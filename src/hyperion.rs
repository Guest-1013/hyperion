use std::f64::consts::PI;

#[derive(Clone)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Hyperion {
    pub L: f64,
    pub ax: f64,
    pub ay: f64,
    pub a: f64,

    pub vx: f64,
    pub vy: f64,
    pub v: f64,

    pub x: f64,
    pub y: f64,
    pub r: f64,

    pub alpha: f64,
    pub omega: f64,
    pub theta: f64,

    pub t: f64
}

pub struct HyperionStates {
    pub states: Vec<Hyperion>,
}

impl Hyperion {
    pub fn new(l: f64, vx0: f64, vy0: f64, x0: f64, y0: f64, omega0: f64, theta0: f64) -> Self {
        Self {
            L: l,
            ax: 0., ay: 0., a: 0.,
            vx: vx0, vy: vy0, v: (vx0.powf(2.)+vy0.powf(2.)).sqrt(),
            x: x0, y: y0, r: (x0.powf(2.) + y0.powf(2.)).sqrt(),
            alpha: 0., omega: omega0, theta: theta0,
            t: 0.
        }
    }

    fn geo_distance(x: f64, y: f64) -> f64 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }

    pub fn iterate(&self, dt: f64) -> Self {
        // Euler-Cromer法
        let ax_new: f64 = -4. * PI.powf(2.) * self.x / self.r.powf(3.);
        let ay_new: f64 = -4. * PI.powf(2.) * self.y / self.r.powf(3.);

        let vx_new: f64 = self.vx + dt * ax_new;
        let vy_new: f64 = self.vy + dt * ay_new;

        let x_new: f64 = self.x + dt * vx_new;
        let y_new: f64 = self.y + dt * vy_new;

        let alpha_new: f64 = -12. * PI.powf(2.) * 
            (self.x * self.theta.sin() - self.y * self.theta.cos()) * 
            (self.x * self.theta.cos() + self.y * self.theta.sin()) /
            self.r.powf(5.);
        let omega_new: f64 = self.omega + dt * alpha_new;
        let theta_new: f64 = self.theta + dt * omega_new;

        let t_new: f64 = self.t + dt;

        Self {
            L: self.L,
            ax: ax_new, ay: ay_new, a: Self::geo_distance(ax_new, ay_new),
            vx: vx_new, vy: vy_new, v: Self::geo_distance(vx_new, vy_new),
            x: x_new, y: y_new, r: Self::geo_distance(x_new, y_new),
            alpha: alpha_new, omega: omega_new, theta: theta_new,
            t: t_new
        }
    }
}