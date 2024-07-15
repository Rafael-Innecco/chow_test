pub struct DpVec {
    pub points: Vec<(f64, f64)>,
}

impl DpVec {
    pub fn x_to_ln(&self) -> DpVec {
       let mut temp: Vec<(f64, f64)> = Vec::new();
        for i in 0..self.points.len(){
            let x = self.points[i].0;
            let y = self.points[i].1;
            temp.push((x.ln(), y));
        }
        DpVec {points: temp}
    }

    pub fn x_to_log(&self) -> DpVec {
        let mut temp: Vec<(f64, f64)> = Vec::new();
        for i in 0..self.points.len(){
            let x = self.points[i].0;
            let y = self.points[i].1;
            temp.push((x.log10(), y));
        }
        DpVec {points: temp}
    }

    pub fn x_to_inv(&self) -> DpVec {
        let mut temp: Vec<(f64, f64)> = Vec::new();
        for i in 0..self.points.len() {
            let x = self.points[i].0;
            let y = self.points[i].1;
            temp.push((1.0 / x, y))
        }
        DpVec {points: temp}
    }

    pub fn y_to_ln(&self) -> DpVec {
        let mut temp: Vec<(f64, f64)> = Vec::new();
        for i in 0..self.points.len() {
            let x = self.points[i].0;
            let y = self.points[i].1;
            temp.push((x, y.ln()));
        }
        DpVec {points: temp}
    }
}

impl DpVec{
    pub fn xn_sum(&self, n: i32) -> f64 {
        let mut sum= 0.0;
        for i in 0..self.points.len(){
            sum += self.points[i].0.powi(n);
        }
        sum
    }

    pub fn y_sum(&self) -> f64 {
        let mut sum= 0.0;
        for i in 0..self.points.len(){
            sum += self.points[i].1;
        }
        sum
    }

    pub fn xy_sum(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.points.len() {
            sum += self.points[i].0 * self.points[i].1;
        }
        sum
    }

    pub fn x2y_sum(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.points.len() {
            sum += self.points[i].0.powi(2) * self.points[i].1;
        }
        sum
    }
}
