fn main() {
    println!("Hello, Moon Lander!");
}

const G_Lunaire: f64 = 1.622;

type Altitude = f64;
type ModuleLunaire = Altitude;

fn make_module_lunaire(altitude_initiale: Altitude) -> ModuleLunaire {
    altitude_initiale
}

fn altitude(module: ModuleLunaire) -> Altitude {
    module
}

fn tic_tac(module: ModuleLunaire) -> ModuleLunaire {
    module - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALTITUDE_INITIALE: Altitude = 50.0;

    #[test]
    fn altitude_initiale_est_50_kilometres() {
        let module = make_module_lunaire(ALTITUDE_INITIALE);
        assert_eq!(ALTITUDE_INITIALE, altitude(module));
    }

    #[test]
    fn altitude_change_avec_le_temps_et_la_gravite() {
        let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
        let module_v2 = tic_tac(module_v1);
        let module_v3 = tic_tac(module_v2);

        assert!(altitude(module_v3) < altitude(module_v2));
    }
}
