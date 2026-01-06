struct Contact<'a> {
    nom: &'a str,
    telephone: &'a str,
    email: &'a str,
}

struct CarnetAdresses<'a> {
    contacts: Vec<Contact<'a>>,
}

impl<'a> CarnetAdresses<'a> {
    fn nouveau() -> Self {
        CarnetAdresses {
            contacts: Vec::new(),
        }
    }

    fn ajouter_contact(&mut self, contact: Contact<'a>) {
        self.contacts.push(contact);
    }

    fn chercher_par_nom(&self, nom: &str) -> Option<&Contact<'a>> {
        self.contacts.iter().find(|c| c.nom == nom)
    }

    fn afficher_tous(&self) {
        for contact in &self.contacts {
            println!("{}: {} ({})", contact.nom, contact.telephone, contact.email);
        }
    }
}

fn main() {
    let nom1 = String::from("Alice Dupont");
    let tel1 = String::from("0123456789");
    let email1 = String::from("alice@example.com");

    let mut carnet = CarnetAdresses::nouveau();

    carnet.ajouter_contact(Contact {
        nom: &nom1,
        telephone: &tel1,
        email: &email1,
    });

    carnet.afficher_tous();

    if let Some(contact) = carnet.chercher_par_nom("Alice Dupont") {
        println!("Trouvé : {}", contact.telephone);
    }
}
