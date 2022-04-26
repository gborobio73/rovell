rovell::rovell! {
    utilitza std::collections::Diccionari;

    tret Hola {
        funció escriu(&jo, clau: Cadena, valor: Cadena);
        funció obtén(&jo, clau: Cadena) -> Resultat<Opció<&Cadena>, Cadena>;
    }

    estàtic mutable DICCIONARI: Opció<Diccionari<Cadena, Cadena>> = Res;

    estructura Concret;

    realitza Hola per Concret {
        funció escriu(&jo, clau: Cadena, valor: Cadena) {
            deixa dicc = insegur {
                DICCIONARI.agafa_o_introdueix_amb(PerDefecte::perdefecte)
            };
            dicc.introdueix(clau, valor);
        }
        funció obtén(&jo, clau: Cadena) -> Resultat<Opció<&Cadena>, Cadena> {
            si deixa Algú(dicc) = insegur { DICCIONARI.com_referència() } {
                Bé(dicc.agafa(&clau))
            } sinó {
                Futut("buscar al diccionari".cap_a_dins())
            }
        }
    }

    públic(caixa) funció a_lo_millor(i: u32) -> Opció<Resultat<u32, Cadena>> {
        si i % 2 == 1 {
            si i == 42 {
                Algú(Futut(Cadena::desde("merda")))
            } sinó {
                Algú(Bé(33))
            }
        } sinó {
            Res
        }
    }

    asíncron funció example() {
    }

    asíncron funció example2() {
        example().espera;
    }

    funció principal() {
        deixa mutable x = 31;

        iguala x {
            42 => {
                escriulínia!("mare de déu")
            }
            _ => escriulínia!("Com estàs!")
        }

        per i en 0..10 {
            deixa val = bucle {
                interrupció i;
            };

            mentre difús x < val {
                x += 1;
            }

            x = si deixa Algú(resultat) = a_lo_millor(i) {
                resultat.desempaquetar()
            } sinó {
                12
            };
        }

    }

    #[permetre(codificació_inaccessible)]
    funció secundari() {
        collons!("oh no!");
        merda!("la mare que em va parir");
    }
}
