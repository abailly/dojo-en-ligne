fn main() {
    process();
}

// --- commands -->
// <--- output ----
fn process() {
    let mut module = make_module_lunaire(ALTITUDE_INITIALE);
    while (altitude(&module) > 0.0) {
        println!("{}", altitude(&module));
        module = evolue(&module);
    }
}

const G_Lunaire: f64 = 1.622;
const ALTITUDE_INITIALE: Altitude = 50.0;

#[derive(Clone,Copy,Debug,PartialEq)]
enum EtatMoteur {
   MoteurAllume,
   MoteurEteint
}

type Altitude = f64;
struct ModuleLunaire {
    altitude:  Altitude,
    etat_moteur: EtatMoteur,
}

fn make_module_lunaire(altitude_initiale: Altitude) -> ModuleLunaire {
    ModuleLunaire { altitude: altitude_initiale, etat_moteur: EtatMoteur::MoteurEteint, }
}

fn altitude(module: &ModuleLunaire) -> Altitude {
    module.altitude
}

fn evolue(module: &ModuleLunaire) -> ModuleLunaire {
    if module.etat_moteur == EtatMoteur::MoteurAllume {
        ModuleLunaire { altitude: module.altitude + 1.0,etat_moteur: module.etat_moteur }
    } else {
        ModuleLunaire { altitude: module.altitude - 1.0,etat_moteur: module.etat_moteur }
    }
}

fn allume_moteur(module: &ModuleLunaire) -> ModuleLunaire {
        ModuleLunaire { altitude: module.altitude, etat_moteur: EtatMoteur::MoteurAllume}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn altitude_initiale_est_50_kilometres() {
        let module = make_module_lunaire(ALTITUDE_INITIALE);
        assert_eq!(ALTITUDE_INITIALE, altitude(&module));
    }

    #[test]
    fn moteur_est_initialement_eteint() {
        let module = make_module_lunaire(ALTITUDE_INITIALE);
        EtatMoteur::MoteurAllume; 
        EtatMoteur::MoteurEteint; 
        assert_eq!(EtatMoteur::MoteurEteint, module.etat_moteur);
    }

    #[test]
    fn altitude_change_avec_le_temps_et_la_gravite() {
        let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
        let module_v2 = evolue(&module_v1);
        let module_v3 = evolue(&module_v2);

        assert!(altitude(&module_v3) < altitude(&module_v2));
    }

    #[test]
    fn altitude_change_avec_le_temps_et_le_moteur() {
        let module_v1 = make_module_lunaire(ALTITUDE_INITIALE);
        let module_v2 = allume_moteur(&module_v1);
        let module_v3 = evolue(&module_v2);
        assert!(altitude(&module_v3) > altitude(&module_v2));
    }
}
