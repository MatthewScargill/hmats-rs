use distances::vectors::euclidean;
use num_complex::{Complex, Complex64};
use std::f64::consts::{PI,E};
use scilib::math::bessel::*; // hankel function
use num_complex::ComplexFloat; // complex exponents

// Kernels saved as traits for independence
pub trait Kernel<const D: usize> { 
    fn eval(&self, x: &[f64; D], y: &[f64; D]) -> Complex64; // require generic 2 point eval returning a Complex64
}

// ---------------- LAPLACE KERNEL ----------------------
pub struct Laplace; 

impl<const D: usize> Kernel<D> for Laplace {

    fn eval( &self, x: &[f64; D], y: &[f64; D]) -> Complex64 {
        
        let temp_r: f64 = euclidean(x, y);
        let r: f64 = temp_r.max(1e-15); // must find neater way of dealing with r=0

        // Laplace Green's functions for 2 and 3 dimensions
        if D == 2 { return Complex { re:- (1.0 / (2.0 * std::f64::consts::PI)) * r.ln(), im: 0.0}}
        if D == 3 { return Complex64 { re: 1.0 / (4.0 * std::f64::consts::PI * r), im: 0.0 }}
        else { panic!()} // must be better way of doing this, maybe in Nodes new impl
    }
}

// ------------------ HELMHOLTZ KERNELS ------------------

// ----- Standard ---------

pub struct Helmholtz { pub wavenumber: f64}

// "new" method for ease of setting k -- eg. Helmholtz::new(3.02)
impl Helmholtz { pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}}

impl<const D: usize> Kernel<D> for Helmholtz {

    fn eval( &self, x: &[f64; D], y: &[f64; D]) -> Complex64 {
        
        let temp_r: f64 = euclidean(x, y);
        let r: f64 = temp_r.max(1e-15); // must find neater way of dealing with r=0

        // Helmholtz Green's functions for 2 and 3 dimensions
        if D == 2 {
            let kr: Complex<f64> = Complex64 {re: self.wavenumber * r, im: 0.0};
            let h0: Complex<f64> = h1_nu(0.0,kr); 
            return (Complex64::i()/4.0) * h0 
        }
        if D == 3 {
            let ikr: Complex<f64> = Complex64 {re: 0.0, im: self.wavenumber * r};
            let exponent: Complex<f64> = E.powc(ikr);
            return - (1.0/(4.0 * PI * r)) * exponent
        }
        else { panic!()} // must be better way of doing this, maybe in Nodes new impl
    }
}

// beyond this is not yet working, need clearer picture of nodes structure before willing to put anything down
// ------- Normal Derivative ----------

//pub struct HelmholtzNormal { pub wavenumber: f64} 
//
//// new method for ease of setting k -- eg. HelmholtzNormal::new(3.02)
//impl HelmholtzNormal {
    //pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}
    //} // only really important for multiple calls per k
//
//// implementing Kernel trait for 2D HelmholtzNormal
//impl Kernel<2> for HelmholtzNormal {
//
    //// Green function eval method 
    //fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> Complex64 {
        //// bog standard
        //let dx = x[0] - y[0];
        //let dy = x[1] - y[1];
        //let r2 = dx*dx + dy*dy;
        //let r = r2.max(1e-15).sqrt();
//
        //// kr and hankel stuff needed
        //// let kr = Self.k * r;
        //// let h0 = hankel0_1(kr); find the fast hankel crate and implement 
//
        //Complex64::i() * PI * r * 0.25 
//        
    //}
//}

// implementing Kernel trait for 3D Helmholtz
//impl Kernel<3> for HelmholtzNormal {
//
    //// Green function eval method -- probably need to add new trait about G or dG 
    //fn eval( &self, x: &[f64; 3], y: &[f64; 3]) -> Complex64 {
        //// bog standard
        //let dx = x[0] - y[0];
        //let dy = x[1] - y[1];
        //let dz = x[2] - y[2];
        //let r2 = dx*dx + dy*dy + dz*dz;
        //let r = r2.max(1e-15).sqrt();
//
        //// kr and hankel stuff needed
        //// let kr = Self.k * r;
        //// let h0 = hankel0_1(kr); find the fast hankel crate and implement 
//
        //Complex64::i() * PI * r * 0.25 
//        
    //}
//}
