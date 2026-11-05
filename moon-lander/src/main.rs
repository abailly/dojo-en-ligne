fn main() {
    process();
}

// --- commands -->
// <--- output ----
fn process() {
    let mut module = make_module_lunaire(ALTITUDE_INITIALE);
    while (altitude(module) > 0.0) {
        println!("{}", altitude(module));
        module = evolue(module);
    }
}

const G_Lunaire: f64 = 1.622;
const ALTITUDE_INITIALE: Altitude = 50.0;

type Altitude = f64;
type ModuleLunaire = Altitude;

fn make_module_lunaire(altitude_initiale: Altitude) -> ModuleLunaire {
    altitude_initiale
}

fn altitude(module: ModuleLunaire) -> Altitude {
    module
}

fn evolue(module: ModuleLunaire) -> ModuleLunaire {
    module - 1.0
}

fn allume_moteur(module: ModuleLunaire) -> ModuleLunaire {
  module + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn altitude_initiale_est_50_kilometres() {
        let module = make_module_lunaire(ALTITUDE_INITIALE);
        assert_eq!(ALTITUDE_INITIALE, altitude(module));
    }

    #[test]
    fn altitude_change_avec_le_temps_et_la_gravite() {
        let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
        let module_v2 = evolue(module_v1);
        let module_v3 = evolue(module_v2);

        assert!(altitude(module_v3) < altitude(module_v2));
    }
    #[test]
    fn altitude_change_avec_le_temps_et_le_moteur() {
        let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
        let module_v2 = allume_moteur(module_v1);
        assert!(altitude(module_v2) > altitude(module_v1));
    }
}
