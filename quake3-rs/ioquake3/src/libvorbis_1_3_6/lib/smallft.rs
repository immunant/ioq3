// =============== BEGIN smallft_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drft_lookup {
    pub n: libc::c_int,
    pub trigcache: *mut libc::c_float,
    pub splitcache: *mut libc::c_int,
}
use ::libc;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: *unnormalized* fft transform

********************************************************************/
/* FFT implementation from OggSquish, minus cosine transforms,
 * minus all but radix 2/4 case.  In Vorbis we only need this
 * cut-down version.
 *
 * To do more than just power-of-two sized vectors, see the full
 * version I wrote for NetLib.
 *
 * Note that the packing is a little strange; rather than the FFT r/i
 * packing following R_0, I_n, R_1, I_1, R_2, I_2 ... R_n-1, I_n-1,
 * it follows R_0, R_1, I_1, R_2, I_2 ... R_n-1, I_n-1, I_n like the
 * FORTRAN version
 */

unsafe extern "C" fn drfti1(
    mut n: libc::c_int,
    mut wa: *mut libc::c_float,
    mut ifac: *mut libc::c_int,
) {
    static mut ntryh: [libc::c_int; 4] = [
        4 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
    ];
    static mut tpi: libc::c_float = 6.28318530717958648f32;
    let mut arg: libc::c_float = 0.;
    let mut argh: libc::c_float = 0.;
    let mut argld: libc::c_float = 0.;
    let mut fi: libc::c_float = 0.;
    let mut ntry: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = -(1 as libc::c_int);
    let mut k1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut ib: libc::c_int = 0;
    let mut ld: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut is: libc::c_int = 0;
    let mut nq: libc::c_int = 0;
    let mut nr: libc::c_int = 0;
    let mut ido: libc::c_int = 0;
    let mut ipm: libc::c_int = 0;
    let mut nfm1: libc::c_int = 0;
    let mut nl: libc::c_int = n;
    let mut nf: libc::c_int = 0 as libc::c_int;
    'c_12334: loop {
        j += 1;
        if j < 4 as libc::c_int {
            ntry = ntryh[j as usize]
        } else {
            ntry += 2 as libc::c_int
        }
        loop {
            nq = nl / ntry;
            nr = nl - ntry * nq;
            if nr != 0 as libc::c_int {
                break;
            }
            nf += 1;
            *ifac.offset((nf + 1 as libc::c_int) as isize) = ntry;
            nl = nq;
            if !(ntry != 2 as libc::c_int) {
                if !(nf == 1 as libc::c_int) {
                    i = 1 as libc::c_int;
                    while i < nf {
                        ib = nf - i + 1 as libc::c_int;
                        *ifac.offset((ib + 1 as libc::c_int) as isize) = *ifac.offset(ib as isize);
                        i += 1
                    }
                    *ifac.offset(2 as libc::c_int as isize) = 2 as libc::c_int
                }
            }
            if !(nl != 1 as libc::c_int) {
                break 'c_12334;
            }
        }
    }
    *ifac.offset(0 as libc::c_int as isize) = n;
    *ifac.offset(1 as libc::c_int as isize) = nf;
    argh = tpi / n as libc::c_float;
    is = 0 as libc::c_int;
    nfm1 = nf - 1 as libc::c_int;
    l1 = 1 as libc::c_int;
    if nfm1 == 0 as libc::c_int {
        return;
    }
    k1 = 0 as libc::c_int;
    while k1 < nfm1 {
        ip = *ifac.offset((k1 + 2 as libc::c_int) as isize);
        ld = 0 as libc::c_int;
        l2 = l1 * ip;
        ido = n / l2;
        ipm = ip - 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < ipm {
            ld += l1;
            i = is;
            argld = ld as libc::c_float * argh;
            fi = 0.0f32;
            ii = 2 as libc::c_int;
            while ii < ido {
                fi += 1.0f32;
                arg = fi * argld;
                let fresh0 = i;
                i = i + 1;
                *wa.offset(fresh0 as isize) =
                    crate::stdlib::cos(arg as libc::c_double) as libc::c_float;
                let fresh1 = i;
                i = i + 1;
                *wa.offset(fresh1 as isize) =
                    crate::stdlib::sin(arg as libc::c_double) as libc::c_float;
                ii += 2 as libc::c_int
            }
            is += ido;
            j += 1
        }
        l1 = l2;
        k1 += 1
    }
}

unsafe extern "C" fn fdrffti(
    mut n: libc::c_int,
    mut wsave: *mut libc::c_float,
    mut ifac: *mut libc::c_int,
) {
    if n == 1 as libc::c_int {
        return;
    }
    drfti1(n, wsave.offset(n as isize), ifac);
}

unsafe extern "C" fn dradf2(
    mut ido: libc::c_int,
    mut l1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa1: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ti2: libc::c_float = 0.;
    let mut tr2: libc::c_float = 0.;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    t1 = 0 as libc::c_int;
    t2 = l1 * ido;
    t0 = t2;
    t3 = ido << 1 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        *ch.offset((t1 << 1 as libc::c_int) as isize) =
            *cc.offset(t1 as isize) + *cc.offset(t2 as isize);
        *ch.offset(((t1 << 1 as libc::c_int) + t3 - 1 as libc::c_int) as isize) =
            *cc.offset(t1 as isize) - *cc.offset(t2 as isize);
        t1 += ido;
        t2 += ido;
        k += 1
    }
    if ido < 2 as libc::c_int {
        return;
    }
    if !(ido == 2 as libc::c_int) {
        t1 = 0 as libc::c_int;
        t2 = t0;
        k = 0 as libc::c_int;
        while k < l1 {
            t3 = t2;
            t4 = (t1 << 1 as libc::c_int) + (ido << 1 as libc::c_int);
            t5 = t1;
            t6 = t1 + t1;
            i = 2 as libc::c_int;
            while i < ido {
                t3 += 2 as libc::c_int;
                t4 -= 2 as libc::c_int;
                t5 += 2 as libc::c_int;
                t6 += 2 as libc::c_int;
                tr2 = *wa1.offset((i - 2 as libc::c_int) as isize)
                    * *cc.offset((t3 - 1 as libc::c_int) as isize)
                    + *wa1.offset((i - 1 as libc::c_int) as isize) * *cc.offset(t3 as isize);
                ti2 = *wa1.offset((i - 2 as libc::c_int) as isize) * *cc.offset(t3 as isize)
                    - *wa1.offset((i - 1 as libc::c_int) as isize)
                        * *cc.offset((t3 - 1 as libc::c_int) as isize);
                *ch.offset(t6 as isize) = *cc.offset(t5 as isize) + ti2;
                *ch.offset(t4 as isize) = ti2 - *cc.offset(t5 as isize);
                *ch.offset((t6 - 1 as libc::c_int) as isize) =
                    *cc.offset((t5 - 1 as libc::c_int) as isize) + tr2;
                *ch.offset((t4 - 1 as libc::c_int) as isize) =
                    *cc.offset((t5 - 1 as libc::c_int) as isize) - tr2;
                i += 2 as libc::c_int
            }
            t1 += ido;
            t2 += ido;
            k += 1
        }
        if ido % 2 as libc::c_int == 1 as libc::c_int {
            return;
        }
    }
    t1 = ido;
    t2 = t1 - 1 as libc::c_int;
    t3 = t2;
    t2 += t0;
    k = 0 as libc::c_int;
    while k < l1 {
        *ch.offset(t1 as isize) = -*cc.offset(t2 as isize);
        *ch.offset((t1 - 1 as libc::c_int) as isize) = *cc.offset(t3 as isize);
        t1 += ido << 1 as libc::c_int;
        t2 += ido;
        t3 += ido;
        k += 1
    }
}

unsafe extern "C" fn dradf4(
    mut ido: libc::c_int,
    mut l1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa1: *mut libc::c_float,
    mut wa2: *mut libc::c_float,
    mut wa3: *mut libc::c_float,
) {
    static mut hsqt2: libc::c_float = 0.70710678118654752f32;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut ci2: libc::c_float = 0.;
    let mut ci3: libc::c_float = 0.;
    let mut ci4: libc::c_float = 0.;
    let mut cr2: libc::c_float = 0.;
    let mut cr3: libc::c_float = 0.;
    let mut cr4: libc::c_float = 0.;
    let mut ti1: libc::c_float = 0.;
    let mut ti2: libc::c_float = 0.;
    let mut ti3: libc::c_float = 0.;
    let mut ti4: libc::c_float = 0.;
    let mut tr1: libc::c_float = 0.;
    let mut tr2: libc::c_float = 0.;
    let mut tr3: libc::c_float = 0.;
    let mut tr4: libc::c_float = 0.;
    t0 = l1 * ido;
    t1 = t0;
    t4 = t1 << 1 as libc::c_int;
    t2 = t1 + (t1 << 1 as libc::c_int);
    t3 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        tr1 = *cc.offset(t1 as isize) + *cc.offset(t2 as isize);
        tr2 = *cc.offset(t3 as isize) + *cc.offset(t4 as isize);
        t5 = t3 << 2 as libc::c_int;
        *ch.offset(t5 as isize) = tr1 + tr2;
        *ch.offset(((ido << 2 as libc::c_int) + t5 - 1 as libc::c_int) as isize) = tr2 - tr1;
        t5 += ido << 1 as libc::c_int;
        *ch.offset((t5 - 1 as libc::c_int) as isize) =
            *cc.offset(t3 as isize) - *cc.offset(t4 as isize);
        *ch.offset(t5 as isize) = *cc.offset(t2 as isize) - *cc.offset(t1 as isize);
        t1 += ido;
        t2 += ido;
        t3 += ido;
        t4 += ido;
        k += 1
    }
    if ido < 2 as libc::c_int {
        return;
    }
    if !(ido == 2 as libc::c_int) {
        t1 = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < l1 {
            t2 = t1;
            t4 = t1 << 2 as libc::c_int;
            t6 = ido << 1 as libc::c_int;
            t5 = t6 + t4;
            i = 2 as libc::c_int;
            while i < ido {
                t2 += 2 as libc::c_int;
                t3 = t2;
                t4 += 2 as libc::c_int;
                t5 -= 2 as libc::c_int;
                t3 += t0;
                cr2 = *wa1.offset((i - 2 as libc::c_int) as isize)
                    * *cc.offset((t3 - 1 as libc::c_int) as isize)
                    + *wa1.offset((i - 1 as libc::c_int) as isize) * *cc.offset(t3 as isize);
                ci2 = *wa1.offset((i - 2 as libc::c_int) as isize) * *cc.offset(t3 as isize)
                    - *wa1.offset((i - 1 as libc::c_int) as isize)
                        * *cc.offset((t3 - 1 as libc::c_int) as isize);
                t3 += t0;
                cr3 = *wa2.offset((i - 2 as libc::c_int) as isize)
                    * *cc.offset((t3 - 1 as libc::c_int) as isize)
                    + *wa2.offset((i - 1 as libc::c_int) as isize) * *cc.offset(t3 as isize);
                ci3 = *wa2.offset((i - 2 as libc::c_int) as isize) * *cc.offset(t3 as isize)
                    - *wa2.offset((i - 1 as libc::c_int) as isize)
                        * *cc.offset((t3 - 1 as libc::c_int) as isize);
                t3 += t0;
                cr4 = *wa3.offset((i - 2 as libc::c_int) as isize)
                    * *cc.offset((t3 - 1 as libc::c_int) as isize)
                    + *wa3.offset((i - 1 as libc::c_int) as isize) * *cc.offset(t3 as isize);
                ci4 = *wa3.offset((i - 2 as libc::c_int) as isize) * *cc.offset(t3 as isize)
                    - *wa3.offset((i - 1 as libc::c_int) as isize)
                        * *cc.offset((t3 - 1 as libc::c_int) as isize);
                tr1 = cr2 + cr4;
                tr4 = cr4 - cr2;
                ti1 = ci2 + ci4;
                ti4 = ci2 - ci4;
                ti2 = *cc.offset(t2 as isize) + ci3;
                ti3 = *cc.offset(t2 as isize) - ci3;
                tr2 = *cc.offset((t2 - 1 as libc::c_int) as isize) + cr3;
                tr3 = *cc.offset((t2 - 1 as libc::c_int) as isize) - cr3;
                *ch.offset((t4 - 1 as libc::c_int) as isize) = tr1 + tr2;
                *ch.offset(t4 as isize) = ti1 + ti2;
                *ch.offset((t5 - 1 as libc::c_int) as isize) = tr3 - ti4;
                *ch.offset(t5 as isize) = tr4 - ti3;
                *ch.offset((t4 + t6 - 1 as libc::c_int) as isize) = ti4 + tr3;
                *ch.offset((t4 + t6) as isize) = tr4 + ti3;
                *ch.offset((t5 + t6 - 1 as libc::c_int) as isize) = tr2 - tr1;
                *ch.offset((t5 + t6) as isize) = ti1 - ti2;
                i += 2 as libc::c_int
            }
            t1 += ido;
            k += 1
        }
        if ido & 1 as libc::c_int != 0 {
            return;
        }
    }
    t1 = t0 + ido - 1 as libc::c_int;
    t2 = t1 + (t0 << 1 as libc::c_int);
    t3 = ido << 2 as libc::c_int;
    t4 = ido;
    t5 = ido << 1 as libc::c_int;
    t6 = ido;
    k = 0 as libc::c_int;
    while k < l1 {
        ti1 = -hsqt2 * (*cc.offset(t1 as isize) + *cc.offset(t2 as isize));
        tr1 = hsqt2 * (*cc.offset(t1 as isize) - *cc.offset(t2 as isize));
        *ch.offset((t4 - 1 as libc::c_int) as isize) =
            tr1 + *cc.offset((t6 - 1 as libc::c_int) as isize);
        *ch.offset((t4 + t5 - 1 as libc::c_int) as isize) =
            *cc.offset((t6 - 1 as libc::c_int) as isize) - tr1;
        *ch.offset(t4 as isize) = ti1 - *cc.offset((t1 + t0) as isize);
        *ch.offset((t4 + t5) as isize) = ti1 + *cc.offset((t1 + t0) as isize);
        t1 += ido;
        t2 += ido;
        t4 += t3;
        t6 += ido;
        k += 1
    }
}

unsafe extern "C" fn dradfg(
    mut ido: libc::c_int,
    mut ip: libc::c_int,
    mut l1: libc::c_int,
    mut idl1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut c1: *mut libc::c_float,
    mut c2: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut ch2: *mut libc::c_float,
    mut wa: *mut libc::c_float,
) {
    static mut tpi: libc::c_float = 6.283185307179586f32;
    let mut idij: libc::c_int = 0;
    let mut ipph: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ic: libc::c_int = 0;
    let mut ik: libc::c_int = 0;
    let mut is: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut t7: libc::c_int = 0;
    let mut t8: libc::c_int = 0;
    let mut t9: libc::c_int = 0;
    let mut t10: libc::c_int = 0;
    let mut dc2: libc::c_float = 0.;
    let mut ai1: libc::c_float = 0.;
    let mut ai2: libc::c_float = 0.;
    let mut ar1: libc::c_float = 0.;
    let mut ar2: libc::c_float = 0.;
    let mut ds2: libc::c_float = 0.;
    let mut nbd: libc::c_int = 0;
    let mut dcp: libc::c_float = 0.;
    let mut arg: libc::c_float = 0.;
    let mut dsp: libc::c_float = 0.;
    let mut ar1h: libc::c_float = 0.;
    let mut ar2h: libc::c_float = 0.;
    let mut idp2: libc::c_int = 0;
    let mut ipp2: libc::c_int = 0;
    arg = tpi / ip as libc::c_float;
    dcp = crate::stdlib::cos(arg as libc::c_double) as libc::c_float;
    dsp = crate::stdlib::sin(arg as libc::c_double) as libc::c_float;
    ipph = ip + 1 as libc::c_int >> 1 as libc::c_int;
    ipp2 = ip;
    idp2 = ido;
    nbd = ido - 1 as libc::c_int >> 1 as libc::c_int;
    t0 = l1 * ido;
    t10 = ip * ido;
    if !(ido == 1 as libc::c_int) {
        ik = 0 as libc::c_int;
        while ik < idl1 {
            *ch2.offset(ik as isize) = *c2.offset(ik as isize);
            ik += 1
        }
        t1 = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j < ip {
            t1 += t0;
            t2 = t1;
            k = 0 as libc::c_int;
            while k < l1 {
                *ch.offset(t2 as isize) = *c1.offset(t2 as isize);
                t2 += ido;
                k += 1
            }
            j += 1
        }
        is = -ido;
        t1 = 0 as libc::c_int;
        if nbd > l1 {
            j = 1 as libc::c_int;
            while j < ip {
                t1 += t0;
                is += ido;
                t2 = -ido + t1;
                k = 0 as libc::c_int;
                while k < l1 {
                    idij = is - 1 as libc::c_int;
                    t2 += ido;
                    t3 = t2;
                    i = 2 as libc::c_int;
                    while i < ido {
                        idij += 2 as libc::c_int;
                        t3 += 2 as libc::c_int;
                        *ch.offset((t3 - 1 as libc::c_int) as isize) = *wa
                            .offset((idij - 1 as libc::c_int) as isize)
                            * *c1.offset((t3 - 1 as libc::c_int) as isize)
                            + *wa.offset(idij as isize) * *c1.offset(t3 as isize);
                        *ch.offset(t3 as isize) = *wa.offset((idij - 1 as libc::c_int) as isize)
                            * *c1.offset(t3 as isize)
                            - *wa.offset(idij as isize)
                                * *c1.offset((t3 - 1 as libc::c_int) as isize);
                        i += 2 as libc::c_int
                    }
                    k += 1
                }
                j += 1
            }
        } else {
            j = 1 as libc::c_int;
            while j < ip {
                is += ido;
                idij = is - 1 as libc::c_int;
                t1 += t0;
                t2 = t1;
                i = 2 as libc::c_int;
                while i < ido {
                    idij += 2 as libc::c_int;
                    t2 += 2 as libc::c_int;
                    t3 = t2;
                    k = 0 as libc::c_int;
                    while k < l1 {
                        *ch.offset((t3 - 1 as libc::c_int) as isize) = *wa
                            .offset((idij - 1 as libc::c_int) as isize)
                            * *c1.offset((t3 - 1 as libc::c_int) as isize)
                            + *wa.offset(idij as isize) * *c1.offset(t3 as isize);
                        *ch.offset(t3 as isize) = *wa.offset((idij - 1 as libc::c_int) as isize)
                            * *c1.offset(t3 as isize)
                            - *wa.offset(idij as isize)
                                * *c1.offset((t3 - 1 as libc::c_int) as isize);
                        t3 += ido;
                        k += 1
                    }
                    i += 2 as libc::c_int
                }
                j += 1
            }
        }
        t1 = 0 as libc::c_int;
        t2 = ipp2 * t0;
        if nbd < l1 {
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                i = 2 as libc::c_int;
                while i < ido {
                    t3 += 2 as libc::c_int;
                    t4 += 2 as libc::c_int;
                    t5 = t3 - ido;
                    t6 = t4 - ido;
                    k = 0 as libc::c_int;
                    while k < l1 {
                        t5 += ido;
                        t6 += ido;
                        *c1.offset((t5 - 1 as libc::c_int) as isize) = *ch
                            .offset((t5 - 1 as libc::c_int) as isize)
                            + *ch.offset((t6 - 1 as libc::c_int) as isize);
                        *c1.offset((t6 - 1 as libc::c_int) as isize) =
                            *ch.offset(t5 as isize) - *ch.offset(t6 as isize);
                        *c1.offset(t5 as isize) = *ch.offset(t5 as isize) + *ch.offset(t6 as isize);
                        *c1.offset(t6 as isize) = *ch.offset((t6 - 1 as libc::c_int) as isize)
                            - *ch.offset((t5 - 1 as libc::c_int) as isize);
                        k += 1
                    }
                    i += 2 as libc::c_int
                }
                j += 1
            }
        } else {
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                k = 0 as libc::c_int;
                while k < l1 {
                    t5 = t3;
                    t6 = t4;
                    i = 2 as libc::c_int;
                    while i < ido {
                        t5 += 2 as libc::c_int;
                        t6 += 2 as libc::c_int;
                        *c1.offset((t5 - 1 as libc::c_int) as isize) = *ch
                            .offset((t5 - 1 as libc::c_int) as isize)
                            + *ch.offset((t6 - 1 as libc::c_int) as isize);
                        *c1.offset((t6 - 1 as libc::c_int) as isize) =
                            *ch.offset(t5 as isize) - *ch.offset(t6 as isize);
                        *c1.offset(t5 as isize) = *ch.offset(t5 as isize) + *ch.offset(t6 as isize);
                        *c1.offset(t6 as isize) = *ch.offset((t6 - 1 as libc::c_int) as isize)
                            - *ch.offset((t5 - 1 as libc::c_int) as isize);
                        i += 2 as libc::c_int
                    }
                    t3 += ido;
                    t4 += ido;
                    k += 1
                }
                j += 1
            }
        }
    }
    ik = 0 as libc::c_int;
    while ik < idl1 {
        *c2.offset(ik as isize) = *ch2.offset(ik as isize);
        ik += 1
    }
    t1 = 0 as libc::c_int;
    t2 = ipp2 * idl1;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += t0;
        t2 -= t0;
        t3 = t1 - ido;
        t4 = t2 - ido;
        k = 0 as libc::c_int;
        while k < l1 {
            t3 += ido;
            t4 += ido;
            *c1.offset(t3 as isize) = *ch.offset(t3 as isize) + *ch.offset(t4 as isize);
            *c1.offset(t4 as isize) = *ch.offset(t4 as isize) - *ch.offset(t3 as isize);
            k += 1
        }
        j += 1
    }
    ar1 = 1.0f32;
    ai1 = 0.0f32;
    t1 = 0 as libc::c_int;
    t2 = ipp2 * idl1;
    t3 = (ip - 1 as libc::c_int) * idl1;
    l = 1 as libc::c_int;
    while l < ipph {
        t1 += idl1;
        t2 -= idl1;
        ar1h = dcp * ar1 - dsp * ai1;
        ai1 = dcp * ai1 + dsp * ar1;
        ar1 = ar1h;
        t4 = t1;
        t5 = t2;
        t6 = t3;
        t7 = idl1;
        ik = 0 as libc::c_int;
        while ik < idl1 {
            let fresh2 = t7;
            t7 = t7 + 1;
            let fresh3 = t4;
            t4 = t4 + 1;
            *ch2.offset(fresh3 as isize) =
                *c2.offset(ik as isize) + ar1 * *c2.offset(fresh2 as isize);
            let fresh4 = t6;
            t6 = t6 + 1;
            let fresh5 = t5;
            t5 = t5 + 1;
            *ch2.offset(fresh5 as isize) = ai1 * *c2.offset(fresh4 as isize);
            ik += 1
        }
        dc2 = ar1;
        ds2 = ai1;
        ar2 = ar1;
        ai2 = ai1;
        t4 = idl1;
        t5 = (ipp2 - 1 as libc::c_int) * idl1;
        j = 2 as libc::c_int;
        while j < ipph {
            t4 += idl1;
            t5 -= idl1;
            ar2h = dc2 * ar2 - ds2 * ai2;
            ai2 = dc2 * ai2 + ds2 * ar2;
            ar2 = ar2h;
            t6 = t1;
            t7 = t2;
            t8 = t4;
            t9 = t5;
            ik = 0 as libc::c_int;
            while ik < idl1 {
                let fresh6 = t8;
                t8 = t8 + 1;
                let fresh7 = t6;
                t6 = t6 + 1;
                *ch2.offset(fresh7 as isize) += ar2 * *c2.offset(fresh6 as isize);
                let fresh8 = t9;
                t9 = t9 + 1;
                let fresh9 = t7;
                t7 = t7 + 1;
                *ch2.offset(fresh9 as isize) += ai2 * *c2.offset(fresh8 as isize);
                ik += 1
            }
            j += 1
        }
        l += 1
    }
    t1 = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += idl1;
        t2 = t1;
        ik = 0 as libc::c_int;
        while ik < idl1 {
            let fresh10 = t2;
            t2 = t2 + 1;
            *ch2.offset(ik as isize) += *c2.offset(fresh10 as isize);
            ik += 1
        }
        j += 1
    }
    if ido < l1 {
        i = 0 as libc::c_int;
        while i < ido {
            t1 = i;
            t2 = i;
            k = 0 as libc::c_int;
            while k < l1 {
                *cc.offset(t2 as isize) = *ch.offset(t1 as isize);
                t1 += ido;
                t2 += t10;
                k += 1
            }
            i += 1
        }
    } else {
        t1 = 0 as libc::c_int;
        t2 = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < l1 {
            t3 = t1;
            t4 = t2;
            i = 0 as libc::c_int;
            while i < ido {
                let fresh11 = t3;
                t3 = t3 + 1;
                let fresh12 = t4;
                t4 = t4 + 1;
                *cc.offset(fresh12 as isize) = *ch.offset(fresh11 as isize);
                i += 1
            }
            t1 += ido;
            t2 += t10;
            k += 1
        }
    }
    t1 = 0 as libc::c_int;
    t2 = ido << 1 as libc::c_int;
    t3 = 0 as libc::c_int;
    t4 = ipp2 * t0;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += t2;
        t3 += t0;
        t4 -= t0;
        t5 = t1;
        t6 = t3;
        t7 = t4;
        k = 0 as libc::c_int;
        while k < l1 {
            *cc.offset((t5 - 1 as libc::c_int) as isize) = *ch.offset(t6 as isize);
            *cc.offset(t5 as isize) = *ch.offset(t7 as isize);
            t5 += t10;
            t6 += ido;
            t7 += ido;
            k += 1
        }
        j += 1
    }
    if ido == 1 as libc::c_int {
        return;
    }
    if nbd < l1 {
        t1 = -ido;
        t3 = 0 as libc::c_int;
        t4 = 0 as libc::c_int;
        t5 = ipp2 * t0;
        j = 1 as libc::c_int;
        while j < ipph {
            t1 += t2;
            t3 += t2;
            t4 += t0;
            t5 -= t0;
            i = 2 as libc::c_int;
            while i < ido {
                t6 = idp2 + t1 - i;
                t7 = i + t3;
                t8 = i + t4;
                t9 = i + t5;
                k = 0 as libc::c_int;
                while k < l1 {
                    *cc.offset((t7 - 1 as libc::c_int) as isize) = *ch
                        .offset((t8 - 1 as libc::c_int) as isize)
                        + *ch.offset((t9 - 1 as libc::c_int) as isize);
                    *cc.offset((t6 - 1 as libc::c_int) as isize) = *ch
                        .offset((t8 - 1 as libc::c_int) as isize)
                        - *ch.offset((t9 - 1 as libc::c_int) as isize);
                    *cc.offset(t7 as isize) = *ch.offset(t8 as isize) + *ch.offset(t9 as isize);
                    *cc.offset(t6 as isize) = *ch.offset(t9 as isize) - *ch.offset(t8 as isize);
                    t6 += t10;
                    t7 += t10;
                    t8 += ido;
                    t9 += ido;
                    k += 1
                }
                i += 2 as libc::c_int
            }
            j += 1
        }
        return;
    } else {
        t1 = -ido;
        t3 = 0 as libc::c_int;
        t4 = 0 as libc::c_int;
        t5 = ipp2 * t0;
        j = 1 as libc::c_int;
        while j < ipph {
            t1 += t2;
            t3 += t2;
            t4 += t0;
            t5 -= t0;
            t6 = t1;
            t7 = t3;
            t8 = t4;
            t9 = t5;
            k = 0 as libc::c_int;
            while k < l1 {
                i = 2 as libc::c_int;
                while i < ido {
                    ic = idp2 - i;
                    *cc.offset((i + t7 - 1 as libc::c_int) as isize) = *ch
                        .offset((i + t8 - 1 as libc::c_int) as isize)
                        + *ch.offset((i + t9 - 1 as libc::c_int) as isize);
                    *cc.offset((ic + t6 - 1 as libc::c_int) as isize) = *ch
                        .offset((i + t8 - 1 as libc::c_int) as isize)
                        - *ch.offset((i + t9 - 1 as libc::c_int) as isize);
                    *cc.offset((i + t7) as isize) =
                        *ch.offset((i + t8) as isize) + *ch.offset((i + t9) as isize);
                    *cc.offset((ic + t6) as isize) =
                        *ch.offset((i + t9) as isize) - *ch.offset((i + t8) as isize);
                    i += 2 as libc::c_int
                }
                t6 += t10;
                t7 += t10;
                t8 += ido;
                t9 += ido;
                k += 1
            }
            j += 1
        }
        return;
    };
}

unsafe extern "C" fn drftf1(
    mut n: libc::c_int,
    mut c: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa: *mut libc::c_float,
    mut ifac: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut kh: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut iw: libc::c_int = 0;
    let mut ido: libc::c_int = 0;
    let mut idl1: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut ix3: libc::c_int = 0;
    nf = *ifac.offset(1 as libc::c_int as isize);
    na = 1 as libc::c_int;
    l2 = n;
    iw = n;
    k1 = 0 as libc::c_int;
    while k1 < nf {
        kh = nf - k1;
        ip = *ifac.offset((kh + 1 as libc::c_int) as isize);
        l1 = l2 / ip;
        ido = n / l2;
        idl1 = ido * l1;
        iw -= (ip - 1 as libc::c_int) * ido;
        na = 1 as libc::c_int - na;
        if ip != 4 as libc::c_int {
            if ip != 2 as libc::c_int {
                if ido == 1 as libc::c_int {
                    na = 1 as libc::c_int - na
                }
                if na != 0 as libc::c_int {
                    dradfg(
                        ido,
                        ip,
                        l1,
                        idl1,
                        ch,
                        ch,
                        ch,
                        c,
                        c,
                        wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    );
                    na = 0 as libc::c_int
                } else {
                    dradfg(
                        ido,
                        ip,
                        l1,
                        idl1,
                        c,
                        c,
                        c,
                        ch,
                        ch,
                        wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    );
                    na = 1 as libc::c_int
                }
            } else if na != 0 as libc::c_int {
                dradf2(
                    ido,
                    l1,
                    ch,
                    c,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                );
            } else {
                dradf2(
                    ido,
                    l1,
                    c,
                    ch,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                );
            }
        } else {
            ix2 = iw + ido;
            ix3 = ix2 + ido;
            if na != 0 as libc::c_int {
                dradf4(
                    ido,
                    l1,
                    ch,
                    c,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix3 as isize).offset(-(1 as libc::c_int as isize)),
                );
            } else {
                dradf4(
                    ido,
                    l1,
                    c,
                    ch,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix3 as isize).offset(-(1 as libc::c_int as isize)),
                );
            }
        }
        l2 = l1;
        k1 += 1
    }
    if na == 1 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        *c.offset(i as isize) = *ch.offset(i as isize);
        i += 1
    }
}

unsafe extern "C" fn dradb2(
    mut ido: libc::c_int,
    mut l1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa1: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut ti2: libc::c_float = 0.;
    let mut tr2: libc::c_float = 0.;
    t0 = l1 * ido;
    t1 = 0 as libc::c_int;
    t2 = 0 as libc::c_int;
    t3 = (ido << 1 as libc::c_int) - 1 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        *ch.offset(t1 as isize) = *cc.offset(t2 as isize) + *cc.offset((t3 + t2) as isize);
        *ch.offset((t1 + t0) as isize) = *cc.offset(t2 as isize) - *cc.offset((t3 + t2) as isize);
        t1 += ido;
        t2 = t1 << 1 as libc::c_int;
        k += 1
    }
    if ido < 2 as libc::c_int {
        return;
    }
    if !(ido == 2 as libc::c_int) {
        t1 = 0 as libc::c_int;
        t2 = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < l1 {
            t3 = t1;
            t4 = t2;
            t5 = t4 + (ido << 1 as libc::c_int);
            t6 = t0 + t1;
            i = 2 as libc::c_int;
            while i < ido {
                t3 += 2 as libc::c_int;
                t4 += 2 as libc::c_int;
                t5 -= 2 as libc::c_int;
                t6 += 2 as libc::c_int;
                *ch.offset((t3 - 1 as libc::c_int) as isize) = *cc
                    .offset((t4 - 1 as libc::c_int) as isize)
                    + *cc.offset((t5 - 1 as libc::c_int) as isize);
                tr2 = *cc.offset((t4 - 1 as libc::c_int) as isize)
                    - *cc.offset((t5 - 1 as libc::c_int) as isize);
                *ch.offset(t3 as isize) = *cc.offset(t4 as isize) - *cc.offset(t5 as isize);
                ti2 = *cc.offset(t4 as isize) + *cc.offset(t5 as isize);
                *ch.offset((t6 - 1 as libc::c_int) as isize) =
                    *wa1.offset((i - 2 as libc::c_int) as isize) * tr2
                        - *wa1.offset((i - 1 as libc::c_int) as isize) * ti2;
                *ch.offset(t6 as isize) = *wa1.offset((i - 2 as libc::c_int) as isize) * ti2
                    + *wa1.offset((i - 1 as libc::c_int) as isize) * tr2;
                i += 2 as libc::c_int
            }
            t1 += ido;
            t2 = t1 << 1 as libc::c_int;
            k += 1
        }
        if ido % 2 as libc::c_int == 1 as libc::c_int {
            return;
        }
    }
    t1 = ido - 1 as libc::c_int;
    t2 = ido - 1 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        *ch.offset(t1 as isize) = *cc.offset(t2 as isize) + *cc.offset(t2 as isize);
        *ch.offset((t1 + t0) as isize) = -(*cc.offset((t2 + 1 as libc::c_int) as isize)
            + *cc.offset((t2 + 1 as libc::c_int) as isize));
        t1 += ido;
        t2 += ido << 1 as libc::c_int;
        k += 1
    }
}

unsafe extern "C" fn dradb3(
    mut ido: libc::c_int,
    mut l1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa1: *mut libc::c_float,
    mut wa2: *mut libc::c_float,
) {
    static mut taur: libc::c_float = -0.5f32;
    static mut taui: libc::c_float = 0.8660254037844386f32;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut t7: libc::c_int = 0;
    let mut t8: libc::c_int = 0;
    let mut t9: libc::c_int = 0;
    let mut t10: libc::c_int = 0;
    let mut ci2: libc::c_float = 0.;
    let mut ci3: libc::c_float = 0.;
    let mut di2: libc::c_float = 0.;
    let mut di3: libc::c_float = 0.;
    let mut cr2: libc::c_float = 0.;
    let mut cr3: libc::c_float = 0.;
    let mut dr2: libc::c_float = 0.;
    let mut dr3: libc::c_float = 0.;
    let mut ti2: libc::c_float = 0.;
    let mut tr2: libc::c_float = 0.;
    t0 = l1 * ido;
    t1 = 0 as libc::c_int;
    t2 = t0 << 1 as libc::c_int;
    t3 = ido << 1 as libc::c_int;
    t4 = ido + (ido << 1 as libc::c_int);
    t5 = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        tr2 = *cc.offset((t3 - 1 as libc::c_int) as isize)
            + *cc.offset((t3 - 1 as libc::c_int) as isize);
        cr2 = *cc.offset(t5 as isize) + taur * tr2;
        *ch.offset(t1 as isize) = *cc.offset(t5 as isize) + tr2;
        ci3 = taui * (*cc.offset(t3 as isize) + *cc.offset(t3 as isize));
        *ch.offset((t1 + t0) as isize) = cr2 - ci3;
        *ch.offset((t1 + t2) as isize) = cr2 + ci3;
        t1 += ido;
        t3 += t4;
        t5 += t4;
        k += 1
    }
    if ido == 1 as libc::c_int {
        return;
    }
    t1 = 0 as libc::c_int;
    t3 = ido << 1 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        t7 = t1 + (t1 << 1 as libc::c_int);
        t5 = t7 + t3;
        t6 = t5;
        t8 = t1;
        t9 = t1 + t0;
        t10 = t9 + t0;
        i = 2 as libc::c_int;
        while i < ido {
            t5 += 2 as libc::c_int;
            t6 -= 2 as libc::c_int;
            t7 += 2 as libc::c_int;
            t8 += 2 as libc::c_int;
            t9 += 2 as libc::c_int;
            t10 += 2 as libc::c_int;
            tr2 = *cc.offset((t5 - 1 as libc::c_int) as isize)
                + *cc.offset((t6 - 1 as libc::c_int) as isize);
            cr2 = *cc.offset((t7 - 1 as libc::c_int) as isize) + taur * tr2;
            *ch.offset((t8 - 1 as libc::c_int) as isize) =
                *cc.offset((t7 - 1 as libc::c_int) as isize) + tr2;
            ti2 = *cc.offset(t5 as isize) - *cc.offset(t6 as isize);
            ci2 = *cc.offset(t7 as isize) + taur * ti2;
            *ch.offset(t8 as isize) = *cc.offset(t7 as isize) + ti2;
            cr3 = taui
                * (*cc.offset((t5 - 1 as libc::c_int) as isize)
                    - *cc.offset((t6 - 1 as libc::c_int) as isize));
            ci3 = taui * (*cc.offset(t5 as isize) + *cc.offset(t6 as isize));
            dr2 = cr2 - ci3;
            dr3 = cr2 + ci3;
            di2 = ci2 + cr3;
            di3 = ci2 - cr3;
            *ch.offset((t9 - 1 as libc::c_int) as isize) =
                *wa1.offset((i - 2 as libc::c_int) as isize) * dr2
                    - *wa1.offset((i - 1 as libc::c_int) as isize) * di2;
            *ch.offset(t9 as isize) = *wa1.offset((i - 2 as libc::c_int) as isize) * di2
                + *wa1.offset((i - 1 as libc::c_int) as isize) * dr2;
            *ch.offset((t10 - 1 as libc::c_int) as isize) =
                *wa2.offset((i - 2 as libc::c_int) as isize) * dr3
                    - *wa2.offset((i - 1 as libc::c_int) as isize) * di3;
            *ch.offset(t10 as isize) = *wa2.offset((i - 2 as libc::c_int) as isize) * di3
                + *wa2.offset((i - 1 as libc::c_int) as isize) * dr3;
            i += 2 as libc::c_int
        }
        t1 += ido;
        k += 1
    }
}

unsafe extern "C" fn dradb4(
    mut ido: libc::c_int,
    mut l1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa1: *mut libc::c_float,
    mut wa2: *mut libc::c_float,
    mut wa3: *mut libc::c_float,
) {
    static mut sqrt2: libc::c_float = 1.414213562373095f32;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut t7: libc::c_int = 0;
    let mut t8: libc::c_int = 0;
    let mut ci2: libc::c_float = 0.;
    let mut ci3: libc::c_float = 0.;
    let mut ci4: libc::c_float = 0.;
    let mut cr2: libc::c_float = 0.;
    let mut cr3: libc::c_float = 0.;
    let mut cr4: libc::c_float = 0.;
    let mut ti1: libc::c_float = 0.;
    let mut ti2: libc::c_float = 0.;
    let mut ti3: libc::c_float = 0.;
    let mut ti4: libc::c_float = 0.;
    let mut tr1: libc::c_float = 0.;
    let mut tr2: libc::c_float = 0.;
    let mut tr3: libc::c_float = 0.;
    let mut tr4: libc::c_float = 0.;
    t0 = l1 * ido;
    t1 = 0 as libc::c_int;
    t2 = ido << 2 as libc::c_int;
    t3 = 0 as libc::c_int;
    t6 = ido << 1 as libc::c_int;
    k = 0 as libc::c_int;
    while k < l1 {
        t4 = t3 + t6;
        t5 = t1;
        tr3 = *cc.offset((t4 - 1 as libc::c_int) as isize)
            + *cc.offset((t4 - 1 as libc::c_int) as isize);
        tr4 = *cc.offset(t4 as isize) + *cc.offset(t4 as isize);
        t4 += t6;
        tr1 = *cc.offset(t3 as isize) - *cc.offset((t4 - 1 as libc::c_int) as isize);
        tr2 = *cc.offset(t3 as isize) + *cc.offset((t4 - 1 as libc::c_int) as isize);
        *ch.offset(t5 as isize) = tr2 + tr3;
        t5 += t0;
        *ch.offset(t5 as isize) = tr1 - tr4;
        t5 += t0;
        *ch.offset(t5 as isize) = tr2 - tr3;
        t5 += t0;
        *ch.offset(t5 as isize) = tr1 + tr4;
        t1 += ido;
        t3 += t2;
        k += 1
    }
    if ido < 2 as libc::c_int {
        return;
    }
    if !(ido == 2 as libc::c_int) {
        t1 = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < l1 {
            t2 = t1 << 2 as libc::c_int;
            t3 = t2 + t6;
            t4 = t3;
            t5 = t4 + t6;
            t7 = t1;
            i = 2 as libc::c_int;
            while i < ido {
                t2 += 2 as libc::c_int;
                t3 += 2 as libc::c_int;
                t4 -= 2 as libc::c_int;
                t5 -= 2 as libc::c_int;
                t7 += 2 as libc::c_int;
                ti1 = *cc.offset(t2 as isize) + *cc.offset(t5 as isize);
                ti2 = *cc.offset(t2 as isize) - *cc.offset(t5 as isize);
                ti3 = *cc.offset(t3 as isize) - *cc.offset(t4 as isize);
                tr4 = *cc.offset(t3 as isize) + *cc.offset(t4 as isize);
                tr1 = *cc.offset((t2 - 1 as libc::c_int) as isize)
                    - *cc.offset((t5 - 1 as libc::c_int) as isize);
                tr2 = *cc.offset((t2 - 1 as libc::c_int) as isize)
                    + *cc.offset((t5 - 1 as libc::c_int) as isize);
                ti4 = *cc.offset((t3 - 1 as libc::c_int) as isize)
                    - *cc.offset((t4 - 1 as libc::c_int) as isize);
                tr3 = *cc.offset((t3 - 1 as libc::c_int) as isize)
                    + *cc.offset((t4 - 1 as libc::c_int) as isize);
                *ch.offset((t7 - 1 as libc::c_int) as isize) = tr2 + tr3;
                cr3 = tr2 - tr3;
                *ch.offset(t7 as isize) = ti2 + ti3;
                ci3 = ti2 - ti3;
                cr2 = tr1 - tr4;
                cr4 = tr1 + tr4;
                ci2 = ti1 + ti4;
                ci4 = ti1 - ti4;
                t8 = t7 + t0;
                *ch.offset((t8 - 1 as libc::c_int) as isize) =
                    *wa1.offset((i - 2 as libc::c_int) as isize) * cr2
                        - *wa1.offset((i - 1 as libc::c_int) as isize) * ci2;
                *ch.offset(t8 as isize) = *wa1.offset((i - 2 as libc::c_int) as isize) * ci2
                    + *wa1.offset((i - 1 as libc::c_int) as isize) * cr2;
                t8 += t0;
                *ch.offset((t8 - 1 as libc::c_int) as isize) =
                    *wa2.offset((i - 2 as libc::c_int) as isize) * cr3
                        - *wa2.offset((i - 1 as libc::c_int) as isize) * ci3;
                *ch.offset(t8 as isize) = *wa2.offset((i - 2 as libc::c_int) as isize) * ci3
                    + *wa2.offset((i - 1 as libc::c_int) as isize) * cr3;
                t8 += t0;
                *ch.offset((t8 - 1 as libc::c_int) as isize) =
                    *wa3.offset((i - 2 as libc::c_int) as isize) * cr4
                        - *wa3.offset((i - 1 as libc::c_int) as isize) * ci4;
                *ch.offset(t8 as isize) = *wa3.offset((i - 2 as libc::c_int) as isize) * ci4
                    + *wa3.offset((i - 1 as libc::c_int) as isize) * cr4;
                i += 2 as libc::c_int
            }
            t1 += ido;
            k += 1
        }
        if ido % 2 as libc::c_int == 1 as libc::c_int {
            return;
        }
    }
    t1 = ido;
    t2 = ido << 2 as libc::c_int;
    t3 = ido - 1 as libc::c_int;
    t4 = ido + (ido << 1 as libc::c_int);
    k = 0 as libc::c_int;
    while k < l1 {
        t5 = t3;
        ti1 = *cc.offset(t1 as isize) + *cc.offset(t4 as isize);
        ti2 = *cc.offset(t4 as isize) - *cc.offset(t1 as isize);
        tr1 = *cc.offset((t1 - 1 as libc::c_int) as isize)
            - *cc.offset((t4 - 1 as libc::c_int) as isize);
        tr2 = *cc.offset((t1 - 1 as libc::c_int) as isize)
            + *cc.offset((t4 - 1 as libc::c_int) as isize);
        *ch.offset(t5 as isize) = tr2 + tr2;
        t5 += t0;
        *ch.offset(t5 as isize) = sqrt2 * (tr1 - ti1);
        t5 += t0;
        *ch.offset(t5 as isize) = ti2 + ti2;
        t5 += t0;
        *ch.offset(t5 as isize) = -sqrt2 * (tr1 + ti1);
        t3 += ido;
        t1 += t2;
        t4 += t2;
        k += 1
    }
}

unsafe extern "C" fn dradbg(
    mut ido: libc::c_int,
    mut ip: libc::c_int,
    mut l1: libc::c_int,
    mut idl1: libc::c_int,
    mut cc: *mut libc::c_float,
    mut c1: *mut libc::c_float,
    mut c2: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut ch2: *mut libc::c_float,
    mut wa: *mut libc::c_float,
) {
    static mut tpi: libc::c_float = 6.283185307179586f32;
    let mut idij: libc::c_int = 0;
    let mut ipph: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ik: libc::c_int = 0;
    let mut is: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut t5: libc::c_int = 0;
    let mut t6: libc::c_int = 0;
    let mut t7: libc::c_int = 0;
    let mut t8: libc::c_int = 0;
    let mut t9: libc::c_int = 0;
    let mut t10: libc::c_int = 0;
    let mut t11: libc::c_int = 0;
    let mut t12: libc::c_int = 0;
    let mut dc2: libc::c_float = 0.;
    let mut ai1: libc::c_float = 0.;
    let mut ai2: libc::c_float = 0.;
    let mut ar1: libc::c_float = 0.;
    let mut ar2: libc::c_float = 0.;
    let mut ds2: libc::c_float = 0.;
    let mut nbd: libc::c_int = 0;
    let mut dcp: libc::c_float = 0.;
    let mut arg: libc::c_float = 0.;
    let mut dsp: libc::c_float = 0.;
    let mut ar1h: libc::c_float = 0.;
    let mut ar2h: libc::c_float = 0.;
    let mut ipp2: libc::c_int = 0;
    t10 = ip * ido;
    t0 = l1 * ido;
    arg = tpi / ip as libc::c_float;
    dcp = crate::stdlib::cos(arg as libc::c_double) as libc::c_float;
    dsp = crate::stdlib::sin(arg as libc::c_double) as libc::c_float;
    nbd = ido - 1 as libc::c_int >> 1 as libc::c_int;
    ipp2 = ip;
    ipph = ip + 1 as libc::c_int >> 1 as libc::c_int;
    if ido < l1 {
        t1 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ido {
            t2 = t1;
            t3 = t1;
            k = 0 as libc::c_int;
            while k < l1 {
                *ch.offset(t2 as isize) = *cc.offset(t3 as isize);
                t2 += ido;
                t3 += t10;
                k += 1
            }
            t1 += 1;
            i += 1
        }
    } else {
        t1 = 0 as libc::c_int;
        t2 = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < l1 {
            t3 = t1;
            t4 = t2;
            i = 0 as libc::c_int;
            while i < ido {
                *ch.offset(t3 as isize) = *cc.offset(t4 as isize);
                t3 += 1;
                t4 += 1;
                i += 1
            }
            t1 += ido;
            t2 += t10;
            k += 1
        }
    }
    t1 = 0 as libc::c_int;
    t2 = ipp2 * t0;
    t5 = ido << 1 as libc::c_int;
    t7 = t5;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += t0;
        t2 -= t0;
        t3 = t1;
        t4 = t2;
        t6 = t5;
        k = 0 as libc::c_int;
        while k < l1 {
            *ch.offset(t3 as isize) = *cc.offset((t6 - 1 as libc::c_int) as isize)
                + *cc.offset((t6 - 1 as libc::c_int) as isize);
            *ch.offset(t4 as isize) = *cc.offset(t6 as isize) + *cc.offset(t6 as isize);
            t3 += ido;
            t4 += ido;
            t6 += t10;
            k += 1
        }
        t5 += t7;
        j += 1
    }
    if !(ido == 1 as libc::c_int) {
        if nbd < l1 {
            t1 = 0 as libc::c_int;
            t2 = ipp2 * t0;
            t7 = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                t7 += ido << 1 as libc::c_int;
                t8 = t7;
                t9 = t7;
                i = 2 as libc::c_int;
                while i < ido {
                    t3 += 2 as libc::c_int;
                    t4 += 2 as libc::c_int;
                    t8 += 2 as libc::c_int;
                    t9 -= 2 as libc::c_int;
                    t5 = t3;
                    t6 = t4;
                    t11 = t8;
                    t12 = t9;
                    k = 0 as libc::c_int;
                    while k < l1 {
                        *ch.offset((t5 - 1 as libc::c_int) as isize) = *cc
                            .offset((t11 - 1 as libc::c_int) as isize)
                            + *cc.offset((t12 - 1 as libc::c_int) as isize);
                        *ch.offset((t6 - 1 as libc::c_int) as isize) = *cc
                            .offset((t11 - 1 as libc::c_int) as isize)
                            - *cc.offset((t12 - 1 as libc::c_int) as isize);
                        *ch.offset(t5 as isize) =
                            *cc.offset(t11 as isize) - *cc.offset(t12 as isize);
                        *ch.offset(t6 as isize) =
                            *cc.offset(t11 as isize) + *cc.offset(t12 as isize);
                        t5 += ido;
                        t6 += ido;
                        t11 += t10;
                        t12 += t10;
                        k += 1
                    }
                    i += 2 as libc::c_int
                }
                j += 1
            }
        } else {
            t1 = 0 as libc::c_int;
            t2 = ipp2 * t0;
            t7 = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                t7 += ido << 1 as libc::c_int;
                t8 = t7;
                k = 0 as libc::c_int;
                while k < l1 {
                    t5 = t3;
                    t6 = t4;
                    t9 = t8;
                    t11 = t8;
                    i = 2 as libc::c_int;
                    while i < ido {
                        t5 += 2 as libc::c_int;
                        t6 += 2 as libc::c_int;
                        t9 += 2 as libc::c_int;
                        t11 -= 2 as libc::c_int;
                        *ch.offset((t5 - 1 as libc::c_int) as isize) = *cc
                            .offset((t9 - 1 as libc::c_int) as isize)
                            + *cc.offset((t11 - 1 as libc::c_int) as isize);
                        *ch.offset((t6 - 1 as libc::c_int) as isize) = *cc
                            .offset((t9 - 1 as libc::c_int) as isize)
                            - *cc.offset((t11 - 1 as libc::c_int) as isize);
                        *ch.offset(t5 as isize) =
                            *cc.offset(t9 as isize) - *cc.offset(t11 as isize);
                        *ch.offset(t6 as isize) =
                            *cc.offset(t9 as isize) + *cc.offset(t11 as isize);
                        i += 2 as libc::c_int
                    }
                    t3 += ido;
                    t4 += ido;
                    t8 += t10;
                    k += 1
                }
                j += 1
            }
        }
    }
    ar1 = 1.0f32;
    ai1 = 0.0f32;
    t1 = 0 as libc::c_int;
    t2 = ipp2 * idl1;
    t9 = t2;
    t3 = (ip - 1 as libc::c_int) * idl1;
    l = 1 as libc::c_int;
    while l < ipph {
        t1 += idl1;
        t2 -= idl1;
        ar1h = dcp * ar1 - dsp * ai1;
        ai1 = dcp * ai1 + dsp * ar1;
        ar1 = ar1h;
        t4 = t1;
        t5 = t2;
        t6 = 0 as libc::c_int;
        t7 = idl1;
        t8 = t3;
        ik = 0 as libc::c_int;
        while ik < idl1 {
            let fresh13 = t6;
            t6 = t6 + 1;
            let fresh14 = t7;
            t7 = t7 + 1;
            let fresh15 = t4;
            t4 = t4 + 1;
            *c2.offset(fresh15 as isize) =
                *ch2.offset(fresh13 as isize) + ar1 * *ch2.offset(fresh14 as isize);
            let fresh16 = t8;
            t8 = t8 + 1;
            let fresh17 = t5;
            t5 = t5 + 1;
            *c2.offset(fresh17 as isize) = ai1 * *ch2.offset(fresh16 as isize);
            ik += 1
        }
        dc2 = ar1;
        ds2 = ai1;
        ar2 = ar1;
        ai2 = ai1;
        t6 = idl1;
        t7 = t9 - idl1;
        j = 2 as libc::c_int;
        while j < ipph {
            t6 += idl1;
            t7 -= idl1;
            ar2h = dc2 * ar2 - ds2 * ai2;
            ai2 = dc2 * ai2 + ds2 * ar2;
            ar2 = ar2h;
            t4 = t1;
            t5 = t2;
            t11 = t6;
            t12 = t7;
            ik = 0 as libc::c_int;
            while ik < idl1 {
                let fresh18 = t11;
                t11 = t11 + 1;
                let fresh19 = t4;
                t4 = t4 + 1;
                *c2.offset(fresh19 as isize) += ar2 * *ch2.offset(fresh18 as isize);
                let fresh20 = t12;
                t12 = t12 + 1;
                let fresh21 = t5;
                t5 = t5 + 1;
                *c2.offset(fresh21 as isize) += ai2 * *ch2.offset(fresh20 as isize);
                ik += 1
            }
            j += 1
        }
        l += 1
    }
    t1 = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += idl1;
        t2 = t1;
        ik = 0 as libc::c_int;
        while ik < idl1 {
            let fresh22 = t2;
            t2 = t2 + 1;
            *ch2.offset(ik as isize) += *ch2.offset(fresh22 as isize);
            ik += 1
        }
        j += 1
    }
    t1 = 0 as libc::c_int;
    t2 = ipp2 * t0;
    j = 1 as libc::c_int;
    while j < ipph {
        t1 += t0;
        t2 -= t0;
        t3 = t1;
        t4 = t2;
        k = 0 as libc::c_int;
        while k < l1 {
            *ch.offset(t3 as isize) = *c1.offset(t3 as isize) - *c1.offset(t4 as isize);
            *ch.offset(t4 as isize) = *c1.offset(t3 as isize) + *c1.offset(t4 as isize);
            t3 += ido;
            t4 += ido;
            k += 1
        }
        j += 1
    }
    if !(ido == 1 as libc::c_int) {
        if nbd < l1 {
            t1 = 0 as libc::c_int;
            t2 = ipp2 * t0;
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                i = 2 as libc::c_int;
                while i < ido {
                    t3 += 2 as libc::c_int;
                    t4 += 2 as libc::c_int;
                    t5 = t3;
                    t6 = t4;
                    k = 0 as libc::c_int;
                    while k < l1 {
                        *ch.offset((t5 - 1 as libc::c_int) as isize) =
                            *c1.offset((t5 - 1 as libc::c_int) as isize) - *c1.offset(t6 as isize);
                        *ch.offset((t6 - 1 as libc::c_int) as isize) =
                            *c1.offset((t5 - 1 as libc::c_int) as isize) + *c1.offset(t6 as isize);
                        *ch.offset(t5 as isize) =
                            *c1.offset(t5 as isize) + *c1.offset((t6 - 1 as libc::c_int) as isize);
                        *ch.offset(t6 as isize) =
                            *c1.offset(t5 as isize) - *c1.offset((t6 - 1 as libc::c_int) as isize);
                        t5 += ido;
                        t6 += ido;
                        k += 1
                    }
                    i += 2 as libc::c_int
                }
                j += 1
            }
        } else {
            t1 = 0 as libc::c_int;
            t2 = ipp2 * t0;
            j = 1 as libc::c_int;
            while j < ipph {
                t1 += t0;
                t2 -= t0;
                t3 = t1;
                t4 = t2;
                k = 0 as libc::c_int;
                while k < l1 {
                    t5 = t3;
                    t6 = t4;
                    i = 2 as libc::c_int;
                    while i < ido {
                        t5 += 2 as libc::c_int;
                        t6 += 2 as libc::c_int;
                        *ch.offset((t5 - 1 as libc::c_int) as isize) =
                            *c1.offset((t5 - 1 as libc::c_int) as isize) - *c1.offset(t6 as isize);
                        *ch.offset((t6 - 1 as libc::c_int) as isize) =
                            *c1.offset((t5 - 1 as libc::c_int) as isize) + *c1.offset(t6 as isize);
                        *ch.offset(t5 as isize) =
                            *c1.offset(t5 as isize) + *c1.offset((t6 - 1 as libc::c_int) as isize);
                        *ch.offset(t6 as isize) =
                            *c1.offset(t5 as isize) - *c1.offset((t6 - 1 as libc::c_int) as isize);
                        i += 2 as libc::c_int
                    }
                    t3 += ido;
                    t4 += ido;
                    k += 1
                }
                j += 1
            }
        }
    }
    if ido == 1 as libc::c_int {
        return;
    }
    ik = 0 as libc::c_int;
    while ik < idl1 {
        *c2.offset(ik as isize) = *ch2.offset(ik as isize);
        ik += 1
    }
    t1 = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j < ip {
        t1 += t0;
        t2 = t1;
        k = 0 as libc::c_int;
        while k < l1 {
            *c1.offset(t2 as isize) = *ch.offset(t2 as isize);
            t2 += ido;
            k += 1
        }
        j += 1
    }
    if nbd > l1 {
        is = -ido - 1 as libc::c_int;
        t1 = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j < ip {
            is += ido;
            t1 += t0;
            t2 = t1;
            k = 0 as libc::c_int;
            while k < l1 {
                idij = is;
                t3 = t2;
                i = 2 as libc::c_int;
                while i < ido {
                    idij += 2 as libc::c_int;
                    t3 += 2 as libc::c_int;
                    *c1.offset((t3 - 1 as libc::c_int) as isize) = *wa
                        .offset((idij - 1 as libc::c_int) as isize)
                        * *ch.offset((t3 - 1 as libc::c_int) as isize)
                        - *wa.offset(idij as isize) * *ch.offset(t3 as isize);
                    *c1.offset(t3 as isize) = *wa.offset((idij - 1 as libc::c_int) as isize)
                        * *ch.offset(t3 as isize)
                        + *wa.offset(idij as isize) * *ch.offset((t3 - 1 as libc::c_int) as isize);
                    i += 2 as libc::c_int
                }
                t2 += ido;
                k += 1
            }
            j += 1
        }
        return;
    } else {
        is = -ido - 1 as libc::c_int;
        t1 = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j < ip {
            is += ido;
            t1 += t0;
            idij = is;
            t2 = t1;
            i = 2 as libc::c_int;
            while i < ido {
                t2 += 2 as libc::c_int;
                idij += 2 as libc::c_int;
                t3 = t2;
                k = 0 as libc::c_int;
                while k < l1 {
                    *c1.offset((t3 - 1 as libc::c_int) as isize) = *wa
                        .offset((idij - 1 as libc::c_int) as isize)
                        * *ch.offset((t3 - 1 as libc::c_int) as isize)
                        - *wa.offset(idij as isize) * *ch.offset(t3 as isize);
                    *c1.offset(t3 as isize) = *wa.offset((idij - 1 as libc::c_int) as isize)
                        * *ch.offset(t3 as isize)
                        + *wa.offset(idij as isize) * *ch.offset((t3 - 1 as libc::c_int) as isize);
                    t3 += ido;
                    k += 1
                }
                i += 2 as libc::c_int
            }
            j += 1
        }
        return;
    };
}

unsafe extern "C" fn drftb1(
    mut n: libc::c_int,
    mut c: *mut libc::c_float,
    mut ch: *mut libc::c_float,
    mut wa: *mut libc::c_float,
    mut ifac: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut iw: libc::c_int = 0;
    let mut ix2: libc::c_int = 0;
    let mut ix3: libc::c_int = 0;
    let mut ido: libc::c_int = 0;
    let mut idl1: libc::c_int = 0;
    nf = *ifac.offset(1 as libc::c_int as isize);
    na = 0 as libc::c_int;
    l1 = 1 as libc::c_int;
    iw = 1 as libc::c_int;
    k1 = 0 as libc::c_int;
    while k1 < nf {
        ip = *ifac.offset((k1 + 2 as libc::c_int) as isize);
        l2 = ip * l1;
        ido = n / l2;
        idl1 = ido * l1;
        if ip != 4 as libc::c_int {
            if ip != 2 as libc::c_int {
                if ip != 3 as libc::c_int {
                    /*    The radix five case can be translated later..... */
                    /*    if(ip!=5)goto L112;

                      ix2=iw+ido;
                      ix3=ix2+ido;
                      ix4=ix3+ido;
                      if(na!=0)
                        dradb5(ido,l1,ch,c,wa+iw-1,wa+ix2-1,wa+ix3-1,wa+ix4-1);
                      else
                        dradb5(ido,l1,c,ch,wa+iw-1,wa+ix2-1,wa+ix3-1,wa+ix4-1);
                      na=1-na;
                      goto L115;

                    L112:*/
                    if na != 0 as libc::c_int {
                        dradbg(
                            ido,
                            ip,
                            l1,
                            idl1,
                            ch,
                            ch,
                            ch,
                            c,
                            c,
                            wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                        );
                    } else {
                        dradbg(
                            ido,
                            ip,
                            l1,
                            idl1,
                            c,
                            c,
                            c,
                            ch,
                            ch,
                            wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                        );
                    }
                    if ido == 1 as libc::c_int {
                        na = 1 as libc::c_int - na
                    }
                } else {
                    ix2 = iw + ido;
                    if na != 0 as libc::c_int {
                        dradb3(
                            ido,
                            l1,
                            ch,
                            c,
                            wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                            wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                        );
                    } else {
                        dradb3(
                            ido,
                            l1,
                            c,
                            ch,
                            wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                            wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                        );
                    }
                    na = 1 as libc::c_int - na
                }
            } else {
                if na != 0 as libc::c_int {
                    dradb2(
                        ido,
                        l1,
                        ch,
                        c,
                        wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    );
                } else {
                    dradb2(
                        ido,
                        l1,
                        c,
                        ch,
                        wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    );
                }
                na = 1 as libc::c_int - na
            }
        } else {
            ix2 = iw + ido;
            ix3 = ix2 + ido;
            if na != 0 as libc::c_int {
                dradb4(
                    ido,
                    l1,
                    ch,
                    c,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix3 as isize).offset(-(1 as libc::c_int as isize)),
                );
            } else {
                dradb4(
                    ido,
                    l1,
                    c,
                    ch,
                    wa.offset(iw as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix2 as isize).offset(-(1 as libc::c_int as isize)),
                    wa.offset(ix3 as isize).offset(-(1 as libc::c_int as isize)),
                );
            }
            na = 1 as libc::c_int - na
        }
        l1 = l2;
        iw += (ip - 1 as libc::c_int) * ido;
        k1 += 1
    }
    if na == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        *c.offset(i as isize) = *ch.offset(i as isize);
        i += 1
    }
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2007             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: fft transform

********************************************************************/
#[no_mangle]

pub unsafe extern "C" fn drft_forward(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup,
    mut data: *mut libc::c_float,
) {
    if (*l).n == 1 as libc::c_int {
        return;
    }
    drftf1(
        (*l).n,
        data,
        (*l).trigcache,
        (*l).trigcache.offset((*l).n as isize),
        (*l).splitcache,
    );
}
#[no_mangle]

pub unsafe extern "C" fn drft_backward(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup,
    mut data: *mut libc::c_float,
) {
    if (*l).n == 1 as libc::c_int {
        return;
    }
    drftb1(
        (*l).n,
        data,
        (*l).trigcache,
        (*l).trigcache.offset((*l).n as isize),
        (*l).splitcache,
    );
}
#[no_mangle]

pub unsafe extern "C" fn drft_init(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup,
    mut n: libc::c_int,
) {
    (*l).n = n;
    (*l).trigcache = crate::stdlib::calloc(
        (3 as libc::c_int * n) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as *mut libc::c_float;
    (*l).splitcache = crate::stdlib::calloc(
        32 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    fdrffti(n, (*l).trigcache, (*l).splitcache);
}
#[no_mangle]

pub unsafe extern "C" fn drft_clear(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup,
) {
    if !l.is_null() {
        if !(*l).trigcache.is_null() {
            ::libc::free((*l).trigcache as *mut libc::c_void);
        }
        if !(*l).splitcache.is_null() {
            ::libc::free((*l).splitcache as *mut libc::c_void);
        }
        crate::stdlib::memset(
            l as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup>()
                as libc::c_ulong,
        );
    };
}
