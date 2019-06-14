use uom::si::f64::Time;
use uom::si::time::millisecond;
use uom::si::time::second;
use uom::si::f64::ElectricalResistance as Resistance;
use uom::si::electrical_resistance::megaohm;
use uom::si::f64::ElectricCurrent as Current;
use uom::si::electric_current::nanoampere;
use uom::si::f64::ElectricPotential as Voltage;
use uom::si::electric_potential::millivolt;
use uom::si::f64::Ratio;
use uom::si::ratio::{ratio, percent};

fn main() {
    let ex_delta_v = delta_v_markram_2004(Current::new::<nanoampere>(18.0), Time::new::<millisecond>(3.0));
    println!(
        "excitatory delta V: {:?}.", ex_delta_v
    );
    let ih_delta_v = delta_v_markram_2004(Current::new::<nanoampere>(9.0), Time::new::<millisecond>(6.0));
    println!(
        "inhibitory delta V: {:?}.", ih_delta_v
    );
    // println!("{:?}", ex_delta_v / ih_delta_v);
    // println!("{:?}", Time::new::<millisecond>(6.0).value);
    // println!("{:?}", Time::new::<second>(6.0).value);
    // println!("{:?}", Ratio::new::<ratio>(1.0).value);
    // println!("{:?}", Ratio::new::<percent>(1.0).value);
    // let x: () = Time::new::<millisecond>(6.0);
    // let y: () = Time::new::<second>(6.0);
    // let z: () = Ratio::new::<ratio>(1.0);

    // let ex_delta_v_non_s = delta_v_markram_2004_non_d(0.000000018, 0.003);
    //     println!(
    //     "excitatory delta V withoud dimension check: {:?}.", ex_delta_v_non_s
    // );
    
}

fn delta_v_markram_2004(i_max: Current, tau_s: Time) -> Voltage {
    let r_n = Resistance::new::<megaohm>(1.0);
    let tau_n = Time::new::<millisecond>(30.0);
    r_n * i_max * tau_s / (tau_n - tau_s) * (
        ((tau_n / tau_s).ln() * tau_s / (tau_s - tau_n)).exp()
            - ((tau_n / tau_s).ln() * tau_n / (tau_s - tau_n)).exp()
    )
}

fn delta_v_markram_2004_non_d(i_max: f64, tau_s: f64) -> f64 {
    let r_n = 1_000_000.0;
    let tau_n = 0.03;
    r_n * i_max * tau_s / (tau_n - tau_s) * (
        ((tau_n / tau_s).ln() * tau_s / (tau_s - tau_n)).exp()
            - ((tau_n / tau_s).ln() * tau_n / (tau_s - tau_n)).exp()
    )
}


trait DimensionLess {
    fn exp(self) -> Ratio;
    fn ln(self) -> Ratio;
}

impl DimensionLess for Ratio {
    fn exp(self) -> Ratio {
        Ratio::new::<ratio>(self.value.exp())
    }

    fn ln(self) -> Ratio {
        Ratio::new::<ratio>(self.value.ln())
    }
}

// type ElectricPotential = ElectricPotential<SI<V>, V> = Quantity<Dimension, SI<V>, V>
// type ElectricPotential<U, V> = Quantity<Dimension, U, V>;
// type Dimension = ISQ<P2, P1, N3, N1, Z0, Z0, Z0, dyn Kind>;
// pub struct Quantity<D: ?Sized, U: ?Sized, V> 
// where
//     D: Dimension,
//     U: Units<V>,
//     V: Num + Conversion<V>, 
//  {
//     pub dimension: PhantomData<D>,
//     pub units: PhantomData<U>,
//     pub value: V,
// }



// Quantity<(dyn Dimension<N = Z0,
//                         L = Z0,
//                         T = PInt<UInt<UTerm, B1>>,
//                         Th = Z0, I = Z0, M = Z0, J = Z0, Kind = (dyn Kind + 'static)>
//           + 'static),
// dyn Units<f64, length = meter, mass = kilogram, thermodynamic_temperature = kelvin, time = time::second, amount_of_substance = amount_of_substance::mole, luminous_intensity = luminous_intensity::candela, electric_current = electric_current::ampere>, f64>;

// Quantity<(dyn Dimension<N = Z0,
//                         L = Z0,
//                         T = Z0,
//                         Th = Z0, I = Z0, M = Z0, J = Z0, Kind = (dyn Kind + 'static)>
//           + 'static),
// dyn Units<f64, length = meter, mass = kilogram, thermodynamic_temperature = kelvin, time = time::second, amount_of_substance = amount_of_substance::mole, luminous_intensity = luminous_intensity::candela, electric_current = electric_current::ampere>, f64>
