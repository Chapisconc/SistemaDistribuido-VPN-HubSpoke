use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    tipo: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkerInfo {
    id: String,
    status: String,
}

fn main() {
    println!("\n=== INICIANDO NODO COORDINADOR (RUST) ===");
    
    // Simulación del Coordinador
    let tarea = Task { id: 1, tipo: "Mandelbrot_Sector_Alpha".to_string() };
    println!("[COORDINADOR] Generando Tarea: {:?}", tarea);
    println!("[RED_VPN] Enviando paquete a 10.10.10.2...");

    thread::sleep(time::Duration::from_millis(500));

    // Simulación de respuesta del Worker
    let worker = WorkerInfo { id: "Worker-01".to_string(), status: "Ready".to_string() };
    println!("[WORKER] Tarea recibida en nodo remoto.");
    println!("[WORKER] Procesando... {:?}", worker);
    println!("[COORDINADOR] Confirmación recibida. Handshake completado.\n");
}
