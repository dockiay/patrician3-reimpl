//! SGL line drawing for native map routes, using the existing DLL rasterizer.
use super::Surface;

impl Surface {
    pub fn line(&mut self, points: (f32, f32, f32, f32), clip: Option<[i32; 4]>) {
        let (x1, y1, x2, y2) = points;
        if self.width == 0 || self.height == 0 || ![x1, y1, x2, y2].iter().all(|v| v.is_finite()) {
            return;
        }
        let round = |v: f32| v.round_ties_even() as i32;
        let (w, h) = (self.width as i32, self.height as i32);
        let (left, right) = (round(x1).min(round(x2)).max(0), round(x1).max(round(x2)).min(w - 1));
        let (top, bottom) = (round(y1).min(round(y2)).max(0), round(y1).max(round(y2)).min(h - 1));
        if left > right || top > bottom { return; }
        let bounds = clip.map_or([0, 0, w, h], |[l,t,r,b]| [l.max(0),t.max(0),r.min(w),b.min(h)]);
        if bounds[0] >= bounds[2] || bounds[1] >= bounds[3] { return; }
        let (rw, rh) = ((right-left+1) as usize, (bottom-top+1) as usize);
        let mut region = Vec::with_capacity(rw * rh);
        for y in top..=bottom {
            for x in left..=right {
                let at = (y as usize * self.width + x as usize) * 4;
                let p = &self.rgba[at..at+4];
                region.push(u32::from_le_bytes([p[2],p[1],p[0],p[3]]));
            }
        }
        // Translate both the original endpoints and the clip, retaining the
        // rasterizer's phase when the line starts outside the visible region.
        p3_render::sgl::draw_line_int(
            &mut p3_render::sgl::Surface { pixels: &mut region, pitch: rw },
            [round(x1)-left,round(y1)-top,round(x2)-left,round(y2)-top],
            self.mode, self.color,
            Some([bounds[0]-left,bounds[1]-top,bounds[2]-left,bounds[3]-top]),
        );
        for y in 0..rh {
            for x in 0..rw {
                let at = (((top as usize+y)*self.width)+left as usize+x)*4;
                let [b,g,r,a] = region[y*rw+x].to_le_bytes();
                self.rgba[at..at+4].copy_from_slice(&[r,g,b,a]);
            }
        }
    }
}
