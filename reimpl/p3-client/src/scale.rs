//! Same sharp bilinear filter as the window bridge, with reusable coordinates
//! and two horizontally filtered rows instead of four source reads per output pixel.
#[derive(Default)]
pub(crate) struct Scaler {
    size: (usize, usize, usize, usize),
    xs: Vec<(usize, usize, u32)>,
    ys: Vec<(usize, usize, u32)>,
    top: Vec<u32>,
    bottom: Vec<u32>,
}

fn axis(out: usize, src: usize) -> Vec<(usize, usize, u32)> {
    let s = out as f64 / src as f64;
    (0..out).map(|o| {
        let p = (o as f64 + 0.5) / s - 0.5;
        let i = p.floor();
        let t = (((p-i-0.5)*s+0.5).clamp(0.0,1.0)*256.0).round() as u32;
        ((i.max(0.0) as usize).min(src-1), ((i+1.0).max(0.0) as usize).min(src-1), t)
    }).collect()
}

fn rgb(p: &[u8]) -> u32 { u32::from(p[0])<<16 | u32::from(p[1])<<8 | u32::from(p[2]) }

fn mix(a: u32, b: u32, t: u32) -> u32 {
    // Separate R/B lanes by 16 bits: each weighted sum plus rounding is <65536.
    let rb = ((a & 0xff00ff)*(256-t)+(b & 0xff00ff)*t+0x800080)>>8;
    let g = ((a & 0x00ff00)*(256-t)+(b & 0x00ff00)*t+0x008000)>>8;
    (rb & 0xff00ff) | (g & 0x00ff00)
}

fn horizontal(rgba: &[u8], width: usize, y: usize, xs: &[(usize,usize,u32)], out: &mut [u32]) {
    let row = &rgba[y*width*4..(y+1)*width*4];
    for (d,&(x0,x1,t)) in out.iter_mut().zip(xs) {
        let a=rgb(&row[x0*4..x0*4+4]);
        *d=if t==0 { a } else { mix(a,rgb(&row[x1*4..x1*4+4]),t) };
    }
}

impl Scaler {
    pub(crate) fn draw(&mut self, rgba: &[u8], width: usize, height: usize, out: &mut [u32], pitch: usize, view: (usize,usize,usize,usize)) {
        let (vx,vy,vw,vh)=view;
        if self.size != (width,height,vw,vh) {
            self.size=(width,height,vw,vh);
            self.xs=axis(vw,width);
            self.ys=axis(vh,height);
            self.top.resize(vw,0);
            self.bottom.resize(vw,0);
        }
        // Pixels may change every frame, so row contents are never cached across frames.
        let (mut top_y,mut bottom_y)=(None,None);
        for (y,row) in out.chunks_exact_mut(pitch).enumerate() {
            if y<vy || y>=vy+vh { row.fill(0); continue; }
            row[..vx].fill(0);
            row[vx+vw..].fill(0);
            let (y0,y1,t)=self.ys[y-vy];
            if top_y!=Some(y0) {
                if bottom_y==Some(y0) {
                    std::mem::swap(&mut self.top,&mut self.bottom);
                    std::mem::swap(&mut top_y,&mut bottom_y);
                } else {
                    horizontal(rgba,width,y0,&self.xs,&mut self.top);
                    top_y=Some(y0);
                }
            }
            let target=&mut row[vx..vx+vw];
            if t==0 || y0==y1 { target.copy_from_slice(&self.top); continue; }
            if bottom_y!=Some(y1) {
                horizontal(rgba,width,y1,&self.xs,&mut self.bottom);
                bottom_y=Some(y1);
            }
            for ((d,&a),&b) in target.iter_mut().zip(&self.top).zip(&self.bottom) { *d=mix(a,b,t); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Previous per-pixel filter, kept independent of the packed arithmetic/row cache.
    fn reference(src: &[u8], w: usize, h: usize, ow: usize, oh: usize) -> Vec<u32> {
        let axis = |n:usize,s:usize| -> Vec<(usize,usize,u32)> {
            let scale=n as f64/s as f64;
            (0..n).map(|o| {
                let p=(o as f64+0.5)/scale-0.5;
                let i=p.floor();let f=p-i;
                let t=((f-0.5)*scale+0.5).clamp(0.0,1.0);
                ((i.max(0.0) as usize).min(s-1),((i+1.0).max(0.0) as usize).min(s-1),(t*256.0).round() as u32)
            }).collect()
        };
        let xs=axis(ow,w);let ys=axis(oh,h);
        let mix=|a:u8,b:u8,t:u32| (u32::from(a)*(256-t)+u32::from(b)*t+128)>>8;
        let mut out=vec![];
        for (y0,y1,ty) in ys {
            for &(x0,x1,tx) in &xs {
                let c:[u32;3]=std::array::from_fn(|k| {
                    let top=mix(src[(y0*w+x0)*4+k],src[(y0*w+x1)*4+k],tx);
                    let bot=mix(src[(y1*w+x0)*4+k],src[(y1*w+x1)*4+k],tx);
                    (top*(256-ty)+bot*ty+128)>>8
                });
                out.push(c[0]<<16|c[1]<<8|c[2]);
            }
        }
        out
    }

    #[test]
    fn scaling_matches_previous_filter_and_refreshes_changed_frames() {
        let mut scaler=Scaler::default();
        for (w,h) in [(1,1),(3,5),(8,7),(17,11)] {
            for (ow,oh) in [(1,1),(3,2),(7,13),(16,14),(31,23),(40,30)] {
                for seed in [3,79] {
                    let src:Vec<u8>=(0..w*h*4).map(|i| (i*73+i/3+seed) as u8).collect();
                    let mut out=vec![0xdeadbeef;(ow+4)*(oh+6)];
                    scaler.draw(&src,w,h,&mut out,ow+4,(2,3,ow,oh));
                    let expected=reference(&src,w,h,ow,oh);
                    for y in 0..oh+6 {
                        for x in 0..ow+4 {
                            let want=if (3..3+oh).contains(&y) && (2..2+ow).contains(&x) {expected[(y-3)*ow+x-2]} else {0};
                            assert_eq!(out[y*(ow+4)+x],want,"{w}x{h}->{ow}x{oh}, {x},{y}, frame {seed}");
                        }
                    }
                }
            }
        }
    }
}
