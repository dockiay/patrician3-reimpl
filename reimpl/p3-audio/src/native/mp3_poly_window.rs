//! Ventana polifásica escalar derivada de Mp3dec, x87 CW027F.
//! Generado por reimpl/oracle/generate_mp3_poly_window.py; no modificar a mano.
#![allow(
    unused_variables,
    unused_assignments,
    clippy::identity_op,
    clippy::needless_range_loop
)]
#[rustfmt::skip]
const WINDOW:[u32;544]=[
    0x00000000,0xc0e7fe34,0x4254fe56,0xc2e57e35,0x43fe9e03,0xc4a106be,0x44cd6e65,0xc6126fdb,
    0x46928ddb,0x46126fdb,0x44cd6e65,0x44a106be,0x43fe9e03,0x42e57e35,0x4254fe56,0x40e7fe34,
    0x00000000,0xc0e7fe34,0x4254fe56,0xc2e57e35,0x43fe9e03,0xc4a106be,0x44cd6e65,0xc6126fdb,
    0x46928ddb,0x46126fdb,0x44cd6e65,0x44a106be,0x43fe9e03,0x42e57e35,0x4254fe56,0x40e7fe34,
    0xbe7ffee8,0xc0f7fe00,0x4259fe4c,0xc301befd,0x43f9fe0c,0xc4ac66a7,0x44ba368c,0xc619a6cd,
    0x469276db,0x460b36ea,0x44deee42,0x44959ed5,0x4400eefe,0x42c87e6e,0x424ffe5f,0x40cffe71,
    0xbe7ffee8,0xc0f7fe00,0x4259fe4c,0xc301befd,0x43f9fe0c,0xc4ac66a7,0x44ba368c,0xc619a6cd,
    0x469276db,0x460b36ea,0x44deee42,0x44959ed5,0x4400eefe,0x42c87e6e,0x424ffe5f,0x40cffe71,
    0xbe7ffee8,0xc10bfeef,0x425dfe43,0xc3113ede,0x43f3fe18,0xc4b7b691,0x44a53eb6,0xc620d6be,
    0x469232dc,0x4603fdf8,0x44eebe22,0x448a46eb,0x4401fefc,0x42ad7ea5,0x4249fe6a,0x40bffe82,
    0xbe7ffee8,0xc10bfeef,0x425dfe43,0xc3113ede,0x43f3fe18,0xc4b7b691,0x44a53eb6,0xc620d6be,
    0x469232dc,0x4603fdf8,0x44eebe22,0x448a46eb,0x4401fefc,0x42ad7ea5,0x4249fe6a,0x40bffe82,
    0xbe7ffee8,0xc117fed0,0x4260fe40,0xc3213ebe,0x43ec9e27,0xc4c2e67a,0x448e86e3,0xc627fcb0,
    0x4691c1dc,0x45f9940d,0x44fcde06,0x447dee04,0x44026efb,0x4292feda,0x4243fe79,0x40a7febf,
    0xbe7ffee8,0xc117fed0,0x4260fe40,0xc3213ebe,0x43ec9e27,0xc4c2e67a,0x448e86e3,0xc627fcb0,
    0x4691c1dc,0x45f9940d,0x44fcde06,0x447dee04,0x44026efb,0x4292feda,0x4243fe79,0x40a7febf,
    0xbe7ffee8,0xc123feb2,0x4262fe39,0xc331be9c,0x43e3be38,0xc4cde664,0x446bfe28,0xc62f13a2,
    0x4691235e,0x45eb3e2a,0x4504aef7,0x44678e31,0x44024efb,0x4273fe1a,0x423dfe84,0x4097fed0,
    0xbe7ffee8,0xc123feb2,0x4262fe39,0xc331be9c,0x43e3be38,0xc4cde664,0x446bfe28,0xc62f13a2,
    0x4691235e,0x45eb3e2a,0x4504aef7,0x44678e31,0x44024efb,0x4273fe1a,0x423dfe84,0x4097fed0,
    0xbe7ffee8,0xc133fea0,0x4263fe38,0xc342be7a,0x43d95e4d,0xc4d8b64f,0x44376e91,0xc6361794,
    0x469058df,0x45dd0046,0x450a1eec,0x44516e5d,0x4401aefd,0x4244fe74,0x4236fe90,0x4087fee2,
    0xbe7ffee8,0xc133fea0,0x4263fe38,0xc342be7a,0x43d95e4d,0xc4d8b64f,0x44376e91,0xc6361794,
    0x469058df,0x45dd0046,0x450a1eec,0x44516e5d,0x4401aefd,0x4244fe74,0x4236fe90,0x4087fee2,
    0xbe7ffee8,0xc143fe7e,0x4263fe38,0xc353fe58,0x43cd7e65,0xc4e3363a,0x43fe9e03,0xc63d0486,
    0x468f6261,0x45cee262,0x450ecae2,0x443bbe88,0x44008eff,0x4218fecf,0x422ffea1,0x407ffe1a,
    0xbe7ffee8,0xc143fe7e,0x4263fe38,0xc353fe58,0x43cd7e65,0xc4e3363a,0x43fe9e03,0xc63d0486,
    0x468f6261,0x45cee262,0x450ecae2,0x443bbe88,0x44008eff,0x4218fecf,0x422ffea1,0x407ffe1a,
    0xbefffee8,0xc153fe5b,0x4262fe39,0xc365be34,0x43bfde80,0xc4ed6625,0x43873ef2,0xc643d778,
    0x468e4063,0x45c0ea7e,0x4512b2db,0x44266eb3,0x43fdfe04,0x41ddfe48,0x4228feac,0x405ffe3d,
    0xbefffee8,0xc153fe5b,0x4262fe39,0xc365be34,0x43bfde80,0xc4ed6625,0x43873ef2,0xc643d778,
    0x468e4063,0x45c0ea7e,0x4512b2db,0x44266eb3,0x43fdfe04,0x41ddfe48,0x4228feac,0x405ffe3d,
    0xbefffee8,0xc167fe34,0x425ffe41,0xc377be10,0x43b0be9e,0xc4f72e12,0x418bfee6,0xc64a8b6b,
    0x468cf366,0x45b3209a,0x4515ded4,0x44119edd,0x43fa1e0c,0x418ffee2,0x4220febe,0x404ffe4e,
    0xbefffee8,0xc167fe34,0x425ffe41,0xc377be10,0x43b0be9e,0xc4f72e12,0x418bfee6,0xc64a8b6b,
    0x468cf366,0x45b3209a,0x4515ded4,0x44119edd,0x43fa1e0c,0x418ffee2,0x4220febe,0x404ffe4e,
    0xbefffee8,0xc17bfe0d,0x425cfe44,0xc384fef6,0x439ffec0,0xc50042ff,0xc3797e0c,0xc6511c5e,
    0x468b7ce9,0x45a588b5,0x451856cf,0x43fabe0a,0x43f53e16,0x410ffed9,0x4219fece,0x402ffeb6,
    0xbefffee8,0xc17bfe0d,0x425cfe44,0xc384fef6,0x439ffec0,0xc50042ff,0xc3797e0c,0xc6511c5e,
    0x468b7ce9,0x45a588b5,0x451856cf,0x43fabe0a,0x43f53e16,0x410ffed9,0x4219fece,0x402ffeb6,
    0xbefffee8,0xc187fef3,0x4256fe54,0xc38e1ee4,0x438d5ee5,0xc504aaf7,0xc4049ef7,0xc6578851,
    0x4689ddec,0x45982ad0,0x451a1acc,0x43d37e59,0x43efde20,0x3efffee8,0x4212feda,0x401ffec8,
    0xbefffee8,0xc187fef3,0x4256fe54,0xc38e1ee4,0x438d5ee5,0xc504aaf7,0xc4049ef7,0xc6578851,
    0x4689ddec,0x45982ad0,0x451a1acc,0x43d37e59,0x43efde20,0x3efffee8,0x4212feda,0x401ffec8,
    0xbf3ffe1b,0xc191fedf,0x424ffe5f,0xc3973ed2,0x43727e1b,0xc508caee,0xc44e3e64,0xc65dc844,
    0x46881670,0x458b0cea,0x451b3aca,0x43ad7ea5,0x43e9be2c,0xc0e7fe34,0x420afeeb,0x400ffed9,
    0xbf3ffe1b,0xc191fedf,0x424ffe5f,0xc3973ed2,0x43727e1b,0xc508caee,0xc44e3e64,0xc65dc844,
    0x46881670,0x458b0cea,0x451b3aca,0x43ad7ea5,0x43e9be2c,0xc0e7fe34,0x420afeeb,0x400ffed9,
    0xbf3ffe1b,0xc19dfec1,0x4247fe71,0xc3a05ebf,0x43467e74,0xc50c96e7,0xc48da6e5,0xc663db38,
    0x468628f4,0x457c6a07,0x451bb6c9,0x4388deee,0x43e31e3a,0xc163fe38,0x4203fef7,0x3ffffdd5,
    0xbf3ffe1b,0xc19dfec1,0x4247fe71,0xc3a05ebf,0x43467e74,0xc50c96e7,0xc48da6e5,0xc663db38,
    0x468628f4,0x457c6a07,0x451bb6c9,0x4388deee,0x43e31e3a,0xc163fe38,0x4203fef7,0x3ffffdd5,
    0xbf7ffdd5,0xc1a9feab,0x423cfe85,0xc3a97ead,0x43173ed2,0xc5100ae0,0xc4b5ce94,0xc669bc2d,
    0x46841578,0x45634e39,0x451b9ac9,0x434b7e69,0x43dbde48,0xc1a5feb0,0x41f9fe0f,0x3fdffe81,
    0xbf7ffdd5,0xc1a9feab,0x423cfe85,0xc3a97ead,0x43173ed2,0xc5100ae0,0xc4b5ce94,0xc669bc2d,
    0x46841578,0x45634e39,0x451b9ac9,0x434b7e69,0x43dbde48,0xc1a5feb0,0x41f9fe0f,0x3fdffe81,
    0xbf7ffdd5,0xc1b5fe96,0x4230fe9f,0xc3b27e9b,0x42c8fe6e,0xc5131eda,0xc4df8e41,0xc66f6721,
    0x4681ddfc,0x454ace6a,0x451aeeca,0x43083ef0,0x43d43e58,0xc1d3fe5b,0x41e9fe29,0x3fdffe81,
    0xbf7ffdd5,0xc1b5fe96,0x4230fe9f,0xc3b27e9b,0x42c8fe6e,0xc5131eda,0xc4df8e41,0xc66f6721,
    0x4681ddfc,0x454ace6a,0x451aeeca,0x43083ef0,0x43d43e58,0xc1d3fe5b,0x41e9fe29,0x3fdffe81,
    0xbf9ffec8,0xc1c1fe80,0x4222febc,0xc3bb3e8a,0x4238fe8e,0xc515c2d4,0xc5056ef5,0xc674da16,
    0x467f0802,0x4532fa9a,0x4519b6cd,0x428ffedf,0x43cc3e68,0xc1fdfe02,0x41ddfe48,0x3fbffea5,
    0xbf9ffec8,0xc1c1fe80,0x4222febc,0xc3bb3e8a,0x4238fe8e,0xc515c2d4,0xc5056ef5,0xc674da16,
    0x467f0802,0x4532fa9a,0x4519b6cd,0x428ffedf,0x43cc3e68,0xc1fdfe02,0x41ddfe48,0x3fbffea5,
    0xbf9ffec8,0xc1cffe5f,0x4211fedb,0xc3c3de78,0xc133fea0,0xc517fad0,0xc51bdac8,0xc67a110c,
    0x467a110c,0x451bdac8,0x4517fad0,0x4133fea0,0x43c3de78,0xc211fedb,0x41cffe5f,0x3f9ffec8,
    0xbf9ffec8,0xc1cffe5f,0x4211fedb,0xc3c3de78,0xc133fea0,0xc517fad0,0xc51bdac8,0xc67a110c,
    0x467a110c,0x451bdac8,0x4517fad0,0x4133fea0,0x43c3de78,0xc211fedb,0x41cffe5f,0x3f9ffec8,
];
#[rustfmt::skip]
fn cv(index:usize)->f64 {f64::from(f32::from_bits(WINDOW[index]))}
#[rustfmt::skip]
fn forward(state:&[f32;272], sb:usize, cb:usize)->f64 {
    let v0: f64 = f64::from(state[sb+1]);
    let v1: f64 = v0 * cv(cb+1);
    let v2: f64 = f64::from(state[sb+0]);
    let v3: f64 = v2 * cv(cb+0);
    let v4: f64 = f64::from(state[sb+2]);
    let v5: f64 = f64::from(state[sb+3]);
    let v6: f64 = v5 * cv(cb+3);
    let v7: f64 = v4 * cv(cb+2);
    let v8: f64 = v3 + v7;
    let v9: f64 = v1 + v6;
    let v10: f64 = f64::from(state[sb+4]);
    let v11: f64 = f64::from(state[sb+5]);
    let v12: f64 = v11 * cv(cb+5);
    let v13: f64 = v10 * cv(cb+4);
    let v14: f64 = v8 + v13;
    let v15: f64 = v9 + v12;
    let v16: f64 = f64::from(state[sb+6]);
    let v17: f64 = f64::from(state[sb+7]);
    let v18: f64 = v17 * cv(cb+7);
    let v19: f64 = v16 * cv(cb+6);
    let v20: f64 = v14 + v19;
    let v21: f64 = v15 + v18;
    let v22: f64 = f64::from(state[sb+8]);
    let v23: f64 = f64::from(state[sb+9]);
    let v24: f64 = v23 * cv(cb+9);
    let v25: f64 = v22 * cv(cb+8);
    let v26: f64 = v20 + v25;
    let v27: f64 = v21 + v24;
    let v28: f64 = f64::from(state[sb+10]);
    let v29: f64 = f64::from(state[sb+11]);
    let v30: f64 = v29 * cv(cb+11);
    let v31: f64 = v28 * cv(cb+10);
    let v32: f64 = v26 + v31;
    let v33: f64 = v27 + v30;
    let v34: f64 = f64::from(state[sb+12]);
    let v35: f64 = f64::from(state[sb+13]);
    let v36: f64 = v35 * cv(cb+13);
    let v37: f64 = v34 * cv(cb+12);
    let v38: f64 = v32 + v37;
    let v39: f64 = v33 + v36;
    let v40: f64 = f64::from(state[sb+14]);
    let v41: f64 = f64::from(state[sb+15]);
    let v42: f64 = v41 * cv(cb+15);
    let v43: f64 = v40 * cv(cb+14);
    let v44: f64 = v38 + v43;
    let v45: f64 = v39 + v42;
    let v46: f64 = v45 + v44;
    v46
}
#[rustfmt::skip]
fn center_odd(state:&[f32;272], sb:usize, cb:usize)->f64 {
    let v0: f64 = f64::from(state[sb+2]);
    let v1: f64 = v0 * cv(cb+2);
    let v2: f64 = f64::from(state[sb+0]);
    let v3: f64 = v2 * cv(cb+0);
    let v4: f64 = f64::from(state[sb+4]);
    let v5: f64 = f64::from(state[sb+6]);
    let v6: f64 = v5 * cv(cb+6);
    let v7: f64 = v4 * cv(cb+4);
    let v8: f64 = v3 + v7;
    let v9: f64 = v1 + v6;
    let v10: f64 = f64::from(state[sb+8]);
    let v11: f64 = f64::from(state[sb+10]);
    let v12: f64 = v11 * cv(cb+10);
    let v13: f64 = v10 * cv(cb+8);
    let v14: f64 = v8 + v13;
    let v15: f64 = v9 + v12;
    let v16: f64 = f64::from(state[sb+12]);
    let v17: f64 = f64::from(state[sb+14]);
    let v18: f64 = v17 * cv(cb+14);
    let v19: f64 = v16 * cv(cb+12);
    let v20: f64 = v14 + v19;
    let v21: f64 = v15 + v18;
    let v22: f64 = v21 + v20;
    v22
}
#[rustfmt::skip]
fn center_even(state:&[f32;272], sb:usize, cb:usize)->f64 {
    let v0: f64 = f64::from(state[sb+3]);
    let v1: f64 = v0 * cv(cb+3);
    let v2: f64 = f64::from(state[sb+1]);
    let v3: f64 = v2 * cv(cb+1);
    let v4: f64 = f64::from(state[sb+5]);
    let v5: f64 = f64::from(state[sb+7]);
    let v6: f64 = v5 * cv(cb+7);
    let v7: f64 = v4 * cv(cb+5);
    let v8: f64 = v3 + v7;
    let v9: f64 = v1 + v6;
    let v10: f64 = f64::from(state[sb+9]);
    let v11: f64 = f64::from(state[sb+11]);
    let v12: f64 = v11 * cv(cb+11);
    let v13: f64 = v10 * cv(cb+9);
    let v14: f64 = v8 + v13;
    let v15: f64 = v9 + v12;
    let v16: f64 = f64::from(state[sb+13]);
    let v17: f64 = f64::from(state[sb+15]);
    let v18: f64 = v17 * cv(cb+15);
    let v19: f64 = v16 * cv(cb+13);
    let v20: f64 = v14 + v19;
    let v21: f64 = v15 + v18;
    let v22: f64 = v21 + v20;
    v22
}
#[rustfmt::skip]
fn back_odd(state:&[f32;272], sb:usize, cb:usize)->f64 {
    let v0: f64 = f64::from(state[sb+0]);
    let v1: f64 = v0 * cv(cb+15);
    let v2: f64 = f64::from(state[sb+1]);
    let v3: f64 = v2 * cv(cb+14);
    let v4: f64 = f64::from(state[sb+3]);
    let v5: f64 = f64::from(state[sb+2]);
    let v6: f64 = v5 * cv(cb+13);
    let v7: f64 = v4 * cv(cb+12);
    let v8: f64 = v3 + v7;
    let v9: f64 = v1 + v6;
    let v10: f64 = f64::from(state[sb+5]);
    let v11: f64 = f64::from(state[sb+4]);
    let v12: f64 = v11 * cv(cb+11);
    let v13: f64 = v10 * cv(cb+10);
    let v14: f64 = v8 + v13;
    let v15: f64 = v9 + v12;
    let v16: f64 = f64::from(state[sb+7]);
    let v17: f64 = f64::from(state[sb+6]);
    let v18: f64 = v17 * cv(cb+9);
    let v19: f64 = v16 * cv(cb+8);
    let v20: f64 = v14 + v19;
    let v21: f64 = v15 + v18;
    let v22: f64 = f64::from(state[sb+9]);
    let v23: f64 = f64::from(state[sb+8]);
    let v24: f64 = v23 * cv(cb+7);
    let v25: f64 = v22 * cv(cb+6);
    let v26: f64 = v20 + v25;
    let v27: f64 = v21 + v24;
    let v28: f64 = f64::from(state[sb+11]);
    let v29: f64 = f64::from(state[sb+10]);
    let v30: f64 = v29 * cv(cb+5);
    let v31: f64 = v28 * cv(cb+4);
    let v32: f64 = v26 + v31;
    let v33: f64 = v27 + v30;
    let v34: f64 = f64::from(state[sb+13]);
    let v35: f64 = f64::from(state[sb+12]);
    let v36: f64 = v35 * cv(cb+3);
    let v37: f64 = v34 * cv(cb+2);
    let v38: f64 = v32 + v37;
    let v39: f64 = v33 + v36;
    let v40: f64 = f64::from(state[sb+15]);
    let v41: f64 = f64::from(state[sb+14]);
    let v42: f64 = v41 * cv(cb+1);
    let v43: f64 = v40 * cv(cb+0);
    let v44: f64 = v38 + v43;
    let v45: f64 = v39 + v42;
    let v46: f64 = v44 - v45;
    v46
}
#[rustfmt::skip]
fn back_even(state:&[f32;272], sb:usize, cb:usize)->f64 {
    let v0: f64 = f64::from(state[sb+1]);
    let v1: f64 = v0 * cv(cb+14);
    let v2: f64 = f64::from(state[sb+0]);
    let v3: f64 = v2 * cv(cb+15);
    let v4: f64 = f64::from(state[sb+2]);
    let v5: f64 = f64::from(state[sb+3]);
    let v6: f64 = v5 * cv(cb+12);
    let v7: f64 = v4 * cv(cb+13);
    let v8: f64 = v3 + v7;
    let v9: f64 = v1 + v6;
    let v10: f64 = f64::from(state[sb+4]);
    let v11: f64 = f64::from(state[sb+5]);
    let v12: f64 = v11 * cv(cb+10);
    let v13: f64 = v10 * cv(cb+11);
    let v14: f64 = v8 + v13;
    let v15: f64 = v9 + v12;
    let v16: f64 = f64::from(state[sb+6]);
    let v17: f64 = f64::from(state[sb+7]);
    let v18: f64 = v17 * cv(cb+8);
    let v19: f64 = v16 * cv(cb+9);
    let v20: f64 = v14 + v19;
    let v21: f64 = v15 + v18;
    let v22: f64 = f64::from(state[sb+8]);
    let v23: f64 = f64::from(state[sb+9]);
    let v24: f64 = v23 * cv(cb+6);
    let v25: f64 = v22 * cv(cb+7);
    let v26: f64 = v20 + v25;
    let v27: f64 = v21 + v24;
    let v28: f64 = f64::from(state[sb+10]);
    let v29: f64 = f64::from(state[sb+11]);
    let v30: f64 = v29 * cv(cb+4);
    let v31: f64 = v28 * cv(cb+5);
    let v32: f64 = v26 + v31;
    let v33: f64 = v27 + v30;
    let v34: f64 = f64::from(state[sb+12]);
    let v35: f64 = f64::from(state[sb+13]);
    let v36: f64 = v35 * cv(cb+2);
    let v37: f64 = v34 * cv(cb+3);
    let v38: f64 = v32 + v37;
    let v39: f64 = v33 + v36;
    let v40: f64 = f64::from(state[sb+14]);
    let v41: f64 = f64::from(state[sb+15]);
    let v42: f64 = v41 * cv(cb+0);
    let v43: f64 = v40 * cv(cb+1);
    let v44: f64 = v38 + v43;
    let v45: f64 = v39 + v42;
    let v46: f64 = v44 - v45;
    v46
}
#[rustfmt::skip]
fn quantize(x:f64)->i16 {
    let v=x.round_ties_even();
    let n=if !v.is_finite() || !(-2147483648.0..2147483648.0).contains(&v) {i32::MIN} else {v as i32};
    n.clamp(-32768,32767) as i16
}
#[rustfmt::skip]
pub(super) fn apply(state:&[f32;272], position:usize, phase:bool)->[i16;32] {
    let mut out=[0i16;32];
    for i in 0..16 {out[i]=quantize(forward(state,16*i,16-position+32*i));}
    let cb=528-position;
    out[16]=quantize(if phase {center_odd(state,256,cb)} else {center_even(state,256,cb)});
    for i in 0..15 {let sb=240-16*i;let cb=480+position-32*i;out[17+i]=quantize(if phase {back_odd(state,sb,cb)} else {back_even(state,sb,cb)});}
    out
}
