use std::fs::File;
use std::io::{BufWriter, Write};


fn get_mesh_vector(length: f32, mesh_number: usize) -> Vec<f32> {
    let mut mesh_vector: Vec<f32> = vec![0.0; mesh_number + 1];

    let dx = length / (mesh_number as f32);

    for i in 0..mesh_number + 1 {
        mesh_vector[i] = dx * (i as f32);
    }
    mesh_vector
}

fn get_scalar_mesh(length: f32, mesh_number: usize) -> Vec<f32> {
    let mut scalar_mesh: Vec<f32> = vec![0.0; mesh_number];

    let dx = length / (mesh_number as f32);

    for i in 0..mesh_number {
        scalar_mesh[i] = dx * ((i as f32) + 0.5);
    }
    scalar_mesh
}

fn initialize_two_state(size: usize, left_value: f32, right_value: f32) -> Vec<f32> {
    let mut values = vec![right_value; size];
    values[..size / 2].fill(left_value);

    values
}

// fn plot(x: Vec<f32>, y: Vec<f32>) {
//     // Dans main, après avoir créé mesh et u :
//     let mut figure = Figure::new();

//     figure
//         .axes2d()
//         .set_title("Condition initiale", &[])
//         .set_x_label("x", &[])
//         .set_y_label("u", &[])
//         .set_x_range(Fix(0.0), Fix(1.0))
//         .set_y_range(Fix(0.0), Fix(5.5))
//         .lines(&x, &y, &[Caption("u(x)")]);

//     figure
//         .show()
//         .expect("Impossible de lancer Gnuplot")
//         .wait()
//         .expect("Erreur pendant l'affichage");
// }

fn cfl_condition(u: &Vec<f32>, dx: &f32) -> f32 {
    let u_max: f32 = u.iter().map(|v: &f32| v.abs()).reduce(f32::max).expect("u est vide");
    let cfl_factor: f32 = 0.1;
    dx * cfl_factor / u_max
}

fn update(u: &mut Vec<f32>, dx: &f32, dt: &f32) {
    let n = u.len();

    for i in (1..n).rev() {
        u[i] -= dt / dx * (0.5 * u[i] * u[i] - 0.5 * u[i - 1] * u[i - 1]);
    }
}
fn write_csv(path: &str, x_values: Vec<f32>, y_values: Vec<f32>) -> std::io::Result<()> {
    // Vérifie que chaque position possède une valeur associée.
    // Panique si les longueurs sont différentes.
    assert_eq!(x_values.len(), y_values.len());

    // Crée le fichier, ou efface son contenu s'il existe déjà.
    // Le ? propage une éventuelle erreur à l'appelant.
    // BufWriter regroupe les écritures dans un tampon en mémoire.
    let mut file: BufWriter<File> = BufWriter::new(File::create(path)?);

    // Écrit les noms des colonnes, suivis d'un retour à la ligne.
    writeln!(file, "x,u")?;
    
    // Parcourt les deux tableaux ensemble, sans copier leur contenu.
    // x et y sont des références aux éléments de même indice.
    for (x, y) in x_values.iter().zip(y_values) {
        // Écrit les deux nombres séparés par une virgule.
        // writeln! ajoute automatiquement un retour à la ligne.
        writeln!(file, "{x},{y}")?;
    }

    // Transmet au fichier les données encore présentes dans le tampon.
    // Le ? permet de signaler une éventuelle erreur d'écriture.
    file.flush()?;


    // Signale que toutes les opérations ont réussi.
    Ok(())
}
use std::fs;

fn clear_results() -> std::io::Result<()> {
    for entry in fs::read_dir("src/results")? {
        let path = entry?.path();

        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn main() {
    clear_results().expect("Impossible to empty results folder.");

    let length: f32 = 1.0;
    let mesh_number: usize = 100;
    let mesh: Vec<f32> = get_mesh_vector(length, mesh_number);
    let scalar_mesh: Vec<f32> = get_scalar_mesh(length, mesh_number);

    let mut u: Vec<f32> = initialize_two_state(mesh_number, 5.0, 1.0);

    let mut time: f32 = 0.0;
    let t_max: f32 = 0.1;

    let mut dt_print: f32 = t_max / 5.0;
    let mut t_print: f32 = 0.0 ;
    let mut step: i32 = 0 ;

    let dx: f32 = length / (mesh_number as f32);

    while time < t_max {
        // let dt: f32 = cfl_condition(&u, &dx).min(t_max - time);
        let dt: f32 = (0.00001_f32).min(t_max - time);
        
        update(&mut u, &dx, &dt);

        // dbg!(time);
        if time > t_print {
            let path = format!("src/results/results_{step:04}.csv");
            write_csv(&path, scalar_mesh.clone(), u.clone()).expect("Impossible d'écrire le CSV");
            t_print += dt_print;
        }
        time += dt;
        step += 1;
    }
    // dbg!(time);
    write_csv("src/results.csv", scalar_mesh.clone(), u.clone()).expect("Impossible d'écrire le CSV");
}
