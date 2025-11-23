use num_complex::Complex64;
use std::f64::consts::PI;

// Kernels saved as traits for independence
// D is for dimension of the Kernel, will probably stick to 2 but want to get used to this conceptually
pub trait Kernel<const D: usize> { 
    type Scalar; // return type
    fn eval(&self, x: &[f64; D], y: &[f64; D]) -> Self::Scalar; // generic 2 point eval returning a Scalar
} // traits mean i can just call ::eval(x,y) no matter the kernel or dimension

// main kernel public structures (keeping it 2D for now)
pub struct Laplace2D; // simple 2D Laplace
pub struct Helmholtz2D { pub wavenumber: f64, normal_deriv: bool}

// new method for ease of setting k and normal deriv later on -- eg. Helmholtz2D::new(3.02, false)
impl Helmholtz2D {
    pub fn new(wavenumber: f64, normal_deriv: bool) -> Self { Self {wavenumber, normal_deriv}}
    }

// implementing Kernel trait for Laplace2D struct
impl Kernel<2> for Laplace2D {
    type Scalar = f64; // Laplace stays real so f64

    // Green function eval method
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> f64 {
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        // will need better x=y handling but for now call it e-15
        let r = r2.max(1e-15).sqrt();
        - (1.0 / (2.0 * std::f64::consts::PI)) * r.ln() 
    }
}

// implementing Kernel trait for Helmholtz2D
impl Kernel<2> for Helmholtz2D {
    type Scalar = Complex64; // This one needs to be complex

    // Green function eval method -- probably need to add new trait about G or dG 
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> Complex64 {
        // bog standard
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        let r = r2.max(1e-15).sqrt();

        // kr and hankel stuff needed
        // let kr = Self.k * r;
        // let h0 = hankel0_1(kr); find the fast hankel crate and implement 


        // decided to make the kernel eval require &self, now we separate G and dG inside the eval with a Bool
        if self.normal_deriv {
            // normal derivative of the 2d helmholtz green's function here 
            // x, y, and normals will be taken from Nodes as and when 
            Complex64::i() * PI * r * 0.25
        }

        else {
            // standard 2dH greens function
            Complex64::i() * PI * r * 0.25 
        }
        
    }
}
