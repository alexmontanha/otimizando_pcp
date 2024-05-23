use crate::pcp::MateriaPrima;

mod pcp;

fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");

    let materias_primas: Vec<MateriaPrima> = pcp::MateriaPrima::from_json("materias_primas.json");

    println!("Matérias primas: {:?}", materias_primas);

}
