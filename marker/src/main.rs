//marker trait
trait Closable{}

struct FileResource{
    name: String,
    closed: bool,
}
struct NetworkResource{
    address: String,
    closed: bool,
}
struct DatabaseResource{
    connection_string: String,
    closed: bool,
}
impl Closable for FileResource{}
impl Closable for NetworkResource{}
impl Closable for DatabaseResource{}

// Funktion, die eine Ressource schließt
fn close_resource<T:Closable>(_resource: &mut T){
    // Hier würde die Logik zum Schließen der Ressource stehen
    // Für dieses Beispiel setzen wir einfach ein "closed"-Flag
   println!("Ressource geschlossen.");
}
fn main() {
    let mut file_res = FileResource{name: String::from("data.txt"), closed: false};
    let mut net_res = NetworkResource{address: String::from("127.0.0.1:8080"), closed: false};
    let mut db_res = DatabaseResource{connection_string: String::from("Server=localhost;Database=mydb;"), closed: false};
    
    let mut res_closable:Vec< Box<dyn Closable>> = Vec::new();
   res_closable.push(Box::new(file_res));
res_closable.push(Box::new(net_res));
res_closable.push(Box::new(db_res));
    println!("{:?} resources to close", res_closable);
    
    close_resource(&mut file_res);
    close_resource(&mut net_res);
    close_resource(&mut db_res);
}
