## Refleksi 1: Running original code
![Image 1](image/run3to1.png)

Ini adalah hasil dari run 3 client secara sekaligus pada 1 server. Untuk run server kita bisa menggunakan syntax `cargo run --bin server` dan untuk run client bisa menggunakan syntax `cargo run --bin client`. Disini, saya mengimplementasikan concurrent handling untuk broadcast sekaligus menerima pesan dari client dengan `tokio::select!`. Selain itu, agar message yang dikirimkan dari client tidak dikembalikan ke client yang mengirimkan,, saya menerapkan logic tertentu. Untuk setiap message dari client akan di format dengan `let _ = bcast_tx.send(format!("{addr}: {text}"));`. Kemudian message akan di cek sesuai format ini dengan `if !msg.starts_with(&format!("{addr}:"))` sebelum dikirim ke client. Output yang dihasilkan adalah kita bisa mengirim pesan dari client manapun dan akan di-broadcast oleh server ke semua client lainnya kecuali diri sendiri.

## Refleksi 1: Change port
![Image 2](image/changeport.png)

Saya mengganti menjadi port 8081, karena 8080 sudah terpakai. Disini ketika kita mengubah port dari kedua sisi client dan server, tidak ada perbedaan dengan fungsionalitas sebelumnya. Protocol websocket juga masih sama dan tidak dipengaruhi oleh port number. Protocol websocket ini berjalan di atas tcp transport layer, sedangkan port 8081 hanya sebagai dimana server listen connection. Protokol websocket terdefinisi menggunakan URI ws:// dan tokio-websockets crate. Ketika request HTTP dibuat, akan ada header upgrade handshake menjadi websocket. Baru kemudian websocket akan jalan.