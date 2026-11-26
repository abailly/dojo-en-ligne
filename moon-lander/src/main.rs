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

const G_Lunaire: f64 = 1.622;
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
    ModuleLunaire {
        altitude: evolue_altitude(module),
        vitesse: 1.0,
        ..*module
    }
}

fn evolue_altitude(module: &ModuleLunaire) -> Altitude {
    let vitesse = if module.etat_moteur == EtatMoteur::MoteurAllume {
       0.5 
    } else {
        -1.0
    };
    module.altitude + vitesse
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
        #[test]
        fn vitesse_augmente_quand_moteur_est_allume() {
            let module_init = make_module_lunaire(ALTITUDE_INITIALE);
            let module_allume = allume_moteur(&module_init);
            let module = evolue(&module_allume);
            assert_eq!(1.0, module.vitesse)
        }
        #[test]
        fn vitesse_augmente_encore_plus_quand_moteur_est_allume() {
            let module_init = make_module_lunaire(ALTITUDE_INITIALE);
            let module_allume = allume_moteur(&module_init);
            let module_evolue = allume_moteur(&module_allume);
            let module = evolue(&module_evolue);
            assert_eq!(1.0, module.vitesse)
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
        fn altitude_change_avec_le_temps_et_la_gravite() {
            let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_v2 = evolue(&module_v1);
            let module_v3 = evolue(&module_v2);

            assert!(altitude(&module_v3) < altitude(&module_v2));
        }

        #[test]
        fn altitude_augmente_quand_le_moteur_est_allume_sur_etat_initial() {
            let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_v2 = allume_moteur(&module_v1);
            let module_v3 = evolue(&module_v2);
            assert!(altitude(&module_v3) > altitude(&module_v2));
        }

        #[test]
        fn premier_pas_de_temps_avec_moteur_allume() {
    // premier pas de temps avec moteur allumé:
    // poussée moteur = 1, g = 0.5
            let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
            let module_v2 = allume_moteur(&module_v1);
            let module_v3 = evolue(&module_v2);
    // accelération nette = 1 - 0.5
    // nouvelle vitesse = 0 + 0.5 = 0.5
    // nouvelle altitude = 50.5
            assert_eq!(50.0, altitude(&module_v1));
            assert_eq!(50.5, altitude(&module_v3));
        }   


    }
}
