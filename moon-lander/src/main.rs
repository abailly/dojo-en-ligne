fn main() {
    process();
}

// --- commands -->
// <--- output ----
fn process() {
    let mut module = make_module_lunaire(ALTITUDE_INITIALE);
    while altitude(&module) > 0.0 {
        println!("{}", altitude(&module));
        module = evolue(&module);
    }
}

const G_Lunaire: f64 = 0.5;
const ALTITUDE_INITIALE: Altitude = 50.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum EtatMoteur {
    MoteurAllume,
    MoteurEteint,
}

type Altitude = f64;

struct ModuleLunaire {
    altitude: Altitude,
    etat_moteur: EtatMoteur,
    vitesse: f64,
}

fn make_module_lunaire(altitude_initiale: Altitude) -> ModuleLunaire {
    ModuleLunaire {
        altitude: altitude_initiale,
        etat_moteur: EtatMoteur::MoteurEteint,
        vitesse: 0.0,
    }
}

fn altitude(module: &ModuleLunaire) -> Altitude {
    module.altitude
}

fn evolue(module: &ModuleLunaire) -> ModuleLunaire {
    let acceleration = 1.0 * (if module.etat_moteur == EtatMoteur::MoteurAllume { 1.0 } else { 0.0 }) - G_Lunaire;
    let vitesse = module.vitesse + acceleration;
    let altitude = module.altitude + vitesse;

    ModuleLunaire {
        altitude: altitude,
        vitesse: vitesse,
        ..*module
    }
}

fn allume_moteur(module: &ModuleLunaire) -> ModuleLunaire {
    ModuleLunaire {
        altitude: module.altitude,
        etat_moteur: EtatMoteur::MoteurAllume,
        vitesse: module.vitesse,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moteur_est_initialement_eteint() {
        let module = make_module_lunaire(ALTITUDE_INITIALE);
        assert_eq!(EtatMoteur::MoteurEteint, module.etat_moteur);
    }

    mod vitesse {
        use super::*;

        #[test]
        fn initialement_la_vitesse_est_nulle() {
            let module = make_module_lunaire(ALTITUDE_INITIALE);
            assert_eq!(0.0, module.vitesse)
        }
    }

    mod altitude {
        use super::*;

        #[test]
        fn altitude_initiale_est_50_kilometres() {
            let module = make_module_lunaire(ALTITUDE_INITIALE);
            assert_eq!(ALTITUDE_INITIALE, altitude(&module));
        }

        #[test]
        fn premier_pas_de_temps_avec_moteur_allume() {
            let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_v2 = allume_moteur(&module_v1);
            let module_v3 = evolue(&module_v2);
            assert_eq!(50.5, altitude(&module_v3));
        }  

        #[test]
        fn premier_pas_de_temps_avec_moteur_eteint() {
            let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_v2 = evolue(&module_v1);
            assert_eq!(49.5, altitude(&module_v2));
        }   

        #[test]
        fn trois_pas_de_temps_avec_moteur_eteint() {
            let module_t0 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_t1 = evolue(&module_t0);
            let module_t2 = evolue(&module_t1);
            let module_t3 = evolue(&module_t2);
            assert_eq!(47.0, altitude(&module_t3));
        }
    }
}
