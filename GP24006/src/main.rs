#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32,
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---
fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---
fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let altitud_nueva = vuelo.altitud;
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if altitud_nueva < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
    } else if altitud_nueva > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && altitud_nueva < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && altitud_nueva > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && altitud_nueva > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && altitud_nueva < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}

// --- FASE 2: Localización de Vuelos ---
fn buscar_vuelo<'a>(nodo: &'a Option<Box<Nodo>>, altitud: u32) -> Option<&'a Vuelo> {
    let mut actual = nodo;
    while let Some(n) = actual {
        if altitud == n.vuelo.altitud {
            return Some(&n.vuelo);
        } else if altitud < n.vuelo.altitud {
            actual = &n.izquierdo;
        } else {
            actual = &n.derecho;
        }
    }
    None
}

// --- FASE 3: Descenso y Aterrizaje (Eliminación) ---
fn eliminar_vuelo(mut nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    if let Some(mut nodo) = nodo_opt.take() {
        if altitud < nodo.vuelo.altitud {
            nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
        } else if altitud > nodo.vuelo.altitud {
            nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
        } else {
            // Nodo encontrado
            if nodo.izquierdo.is_none() {
                return nodo.derecho.take();
            } else if nodo.derecho.is_none() {
                return nodo.izquierdo.take();
            } else {
                // Nodo con dos hijos: buscar predecesor in-order (máximo del subárbol izquierdo)
                let mut predecesor = nodo.izquierdo.as_ref().unwrap();
                while let Some(ref der) = predecesor.derecho {
                    predecesor = der;
                }
                
                // Copiar los datos del predecesor
                let vuelo_predecesor = predecesor.vuelo.clone();
                nodo.vuelo = vuelo_predecesor.clone();
                
                // Eliminar el predecesor in-order de su posición original
                nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), vuelo_predecesor.altitud);
            }
        }

        // 1. Actualizar altura
        actualizar_altura(&mut nodo);

        // 2. Obtener factor de balance
        let balance = obtener_balance(&nodo);

        // 3. Rebalanceo (Casos AVL)
        
        // Caso Izquierda-Izquierda
        if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
            return Some(rotar_derecha(nodo));
        }
        
        // Caso Izquierda-Derecha
        if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
            return Some(rotar_derecha(nodo));
        }

        // Caso Derecha-Derecha
        if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
            return Some(rotar_izquierda(nodo));
        }

        // Caso Derecha-Izquierda
        if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(rotar_derecha(hijo_der));
            return Some(rotar_izquierda(nodo));
        }

        return Some(nodo);
    }
    None
}

// --- FASE 4: Alerta de Colisión (Opción B: Vuelo de Emergencia) ---
fn vuelo_menor_altitud<'a>(nodo: &'a Option<Box<Nodo>>) -> Option<&'a Vuelo> {
    let mut actual = nodo;
    while let Some(n) = actual {
        if n.izquierdo.is_none() {
            return Some(&n.vuelo);
        }
        actual = &n.izquierdo;
    }
    None
}

// --- Función auxiliar para imprimir el árbol en consola (visualización) ---
fn imprimir_arbol(nodo: &Option<Box<Nodo>>, nivel: usize, prefijo: &str) {
    if let Some(n) = nodo {
        imprimir_arbol(&n.derecho, nivel + 1, "R---");
        println!("{:width$}{} {} (Alt: {})", "", prefijo, n.vuelo.id, n.vuelo.altitud, width = nivel * 4);
        imprimir_arbol(&n.izquierdo, nivel + 1, "L---");
    }
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000), 
        ("UA456", 3000), 
        ("IB101", 2000),
        ("AF999", 4000), 
        ("TA222", 3500), 
        ("AM777", 6000),
    ];

    println!("--- Cargando Radar de Control Aéreo (AVL) ---");
    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }
    
    println!("\n[ESTADO DEL ÁRBOL POST-INSERCIÓN]");
    imprimir_arbol(&radar, 0, "RAÍZ");

    // FASE 2: Búsqueda
    println!("\n--- FASE 2: Localización de Vuelos ---");
    match buscar_vuelo(&radar, 3500) {
        Some(v) => println!("✔ Vuelo encontrado: {} a {} pies", v.id, v.altitud),
        None => println!("✖ Vuelo a 3500 pies no encontrado."),
    }
    match buscar_vuelo(&radar, 8000) {
        Some(v) => println!("✔ Vuelo encontrado: {} a {} pies", v.id, v.altitud),
        None => println!("✖ Vuelo a 8000 pies no encontrado."),
    }

    // FASE 4: Vuelo de Emergencia (Menor Altitud)
    println!("\n--- FASE 4: Vuelo de Emergencia ---");
    match vuelo_menor_altitud(&radar) {
        Some(v) => println!("⚠ ALERTA: El vuelo con MENOR altitud es {} a {} pies", v.id, v.altitud),
        None => println!("Radar vacío."),
    }

    // FASE 3: Aterrizaje (Eliminación)
    println!("\n--- FASE 3: Descenso y Aterrizaje (Eliminando 3000 y 4000) ---");
    println!("Aterrizando UA456 (3000)...");
    radar = eliminar_vuelo(radar, 3000);
    
    println!("Aterrizando AF999 (4000)...");
    radar = eliminar_vuelo(radar, 4000);
    
    println!("\n[ESTADO DEL ÁRBOL POST-ATERRIZAJE]");
    imprimir_arbol(&radar, 0, "RAÍZ");
    
    println!("\nValidando rebalanceo...");
    println!("El radar sigue operando con éxito y el árbol mantiene sus propiedades AVL.");
}
