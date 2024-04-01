DAO---------------------
package com.ug14.rumahsakit;

import java.sql.*;


public class DAO {
    private String url = "jdbc:sqlite:rumahsakit.db";

    public Connection connect(){
        Connection conn = null;
        try{
            conn = DriverManager.getConnection(url);
//            System.out.println("Koneksi berhasil.");
        }catch (SQLException e){
            System.out.println(e.getMessage());
        }finally {
            try{
                if(conn != null){
                    conn.close();
                }
            } catch (SQLException ex){
                System.out.println(ex.getMessage());
            }
        }
        return conn;
    }

    public void createNewDatabase() {
        try (Connection conn = DriverManager.getConnection(url)) {
            if (conn != null) {
                DatabaseMetaData meta = conn.getMetaData();
                System.out.println("Nama: " + meta.getDriverName());
                System.out.println("Database tercipta");
            }
        }catch (SQLException e) {
            System.out.println(e.getMessage());
        }
    }

    public void createNewTable() {
        // SQL statement for creating a new table
        String sql = "CREATE TABLE IF NOT EXISTS pasien (" + "rm INT PRIMARY KEY," + " nama text NOT NULL," + "usia INT NOT NULL," + "alamat text NOT NULL" + ");";
        String sql1 = "CREATE TABLE IF NOT EXISTS jadwal (" + " idPemeriksa INT PRIMARY KEY," + "rm INT NOT NULL," + "idDokter int NOT NULL," + "idSuster int NOT NULL," + "idPelayanan int NOT NULL," + " status text NOT NULL"  + ");";

        try (Connection conn = DriverManager.getConnection(url);
             Statement stmt = conn.createStatement()) {
            // create a new table
            stmt.execute(sql);
            System.out.println("Tabel pasien tercipta!");
            stmt.execute(sql1);
            System.out.println("Tabel jadwal tercipta!");
        }catch(SQLException e){
                System.out.println(e.getMessage());
            }
        }

        public void showAllTables(){
            try (Connection conn = DriverManager.getConnection(url)){
                ResultSet rs = conn.getMetaData().getTables(null, null, null, null);
                while (rs.next()) {
                    System.out.println(rs.getString("TABLE_NAME"));
                }
            }catch (SQLException e) {
                System.out.println(e.getMessage());
            }
        }



//    public Dokter getDokterById(int idDokter){
//
//    }
//
//    public Suster getSusterById(int idSuster){
//
//    }
//
    public void inputPasien(Pasien pasien){
        String sql = "INSERT INTO pasien values" + "(" + pasien.getRm()+ "," + pasien.getNama() + "," + pasien.getUsia() + "," + pasien.getAlamat() + ");";
        try (Connection conn = DriverManager.getConnection(url);
             Statement stmt = conn.createStatement()) {
            // create a new table
            stmt.execute(sql);
            System.out.println("Data pasien berhasil disimpan!");
        }catch(SQLException e){
            System.out.println(e.getMessage());
        }
    }

//
    public static void getPasienSembuh(){
        String sql = "SELECT rm FROM jadwal WHERE status = 1";
        try (Connection conn = DriverManager.getConnection("jdbc:sqlite:rumahsakit.db");
             Statement S = conn.createStatement()) {
             ResultSet rs = null;
             rs = S.executeQuery(sql);
             while(rs.next()){
                 System.out.println(rs.getInt("rm"));
             }

        }catch(SQLException e){
            System.out.println(e.getMessage());
        }

    }
//
//    public void updateStatusPasien(){
//
//    }

    public static void main(String[] args) {
        // TODO Auto-generated method stub
        DAO data = new DAO();
//        data.createNewDatabase();
        data.connect();
//        data.createNewTable();
//        data.showAllTables();
    }
DOKTER---------------------------------------------
}
package com.ug14.rumahsakit;

public class Dokter {
    private static int idDokter = 0;
    private String nama;
    private String spesialis;
    private String ruangan;

    public Dokter(String nama, String spesialis, String ruangan){
        this.nama=nama;
        this.spesialis=spesialis;
        this.ruangan=ruangan;
        idDokter++;
    }

    public void memeriksa(Pasien pasien, Jadwal jadwal){
        if(jadwal.getPasien() == pasien){
                pasien.setLevelPenyakit(pasien.getLevelPenyakit()-1);
        }else{
            System.out.println("ANDA HARUS MELAKUKAN PROSES PENDAFTARAN TERLEBIH DAHULU DI BAGIAN PELAYANAN");
        }
    }

    public void cekStatus(Pasien pasien, Jadwal jadwal){
        if(pasien.getLevelPenyakit() > 0){
            System.out.println("PASIEN ANDA MASIH SAKIT");
        }else{
            System.out.println("SELAMAT PASIEN SUDAH SEMBUH");
            System.out.println("Status pasien berhasil diubah");
            pasien.setLevelPenyakit(0);
            pasien.setStatus(true);
        }

    }

    public static int getIdDokter() {
        return idDokter;
    }
}
JADWAL-------------------------------------------------
package com.ug14.rumahsakit;

public class Jadwal {
    private static int idPemeriksa = 1;
    private Pasien pasien;
    private Dokter dokter;
    private Suster suster;
    private Pelayanan pelayanan;
    private boolean statusDaftar = false;
    private boolean statusScreening = false;

    public static void setIdPemeriksa(int idPemeriksa) {
        Jadwal.idPemeriksa = idPemeriksa;
    }

    public static int getIdPemeriksa() {
        return idPemeriksa;
    }

    public Pasien getPasien() {
        return pasien;
    }

    public void setPasien(Pasien pasien) {
        this.pasien = pasien;
    }

    public Dokter getDokter() {
        return dokter;
    }

    public void setDokter(Dokter dokter) {
        this.dokter = dokter;
    }

    public Suster getSuster() {
        return suster;
    }

    public void setSuster(Suster suster) {
        this.suster = suster;
    }

    public Pelayanan getPelayanan() {
        return pelayanan;
    }

    public void setPelayanan(Pelayanan pelayanan) {
        this.pelayanan = pelayanan;
    }

    public boolean isStatusDaftar() {
        return statusDaftar;
    }

    public void setStatusDaftar(boolean statusDaftar) {
        this.statusDaftar = statusDaftar;
    }

    public boolean isStatusScreening() {
        return statusScreening;
    }

    public void setStatusScreening(boolean statusScreening) {
        this.statusScreening = statusScreening;
    }
}
PASIEN-------------------------------
package com.ug14.rumahsakit;

public class Pasien {
    private static int rm = 0;
    private String nama;
    private int usia;
    private String alamat;
    private String penyakit;
    private int levelPenyakit = 3;
    private boolean status = false;

    public Pasien(String nama, int usia, String alamat){
        this.nama = nama;
        this.usia = usia;
        this.alamat = alamat;
        Pasien.rm++;
    }

    public static int getRm() {
        return rm;
    }

    public String getNama() {
        return nama;
    }

    public int getUsia() {
        return usia;
    }

    public String getAlamat() {
        return alamat;
    }

    public boolean getStatus(){
        return this.status;
    }

    public void setStatus(boolean condition){
        this.status = condition;
    }

    public int getLevelPenyakit() {
        return levelPenyakit;
    }

    public void setLevelPenyakit(int levelPenyakit) {
        this.levelPenyakit = levelPenyakit;
    }
}
PELAYANAN------------------------------
package com.ug14.rumahsakit;

import java.util.Scanner;

public class Pelayanan {
    private int idPelayan;
    private String nama;

    public void mengaturJadwal(Pasien pasien, Dokter dokter, Suster suster, Jadwal jadwal){
        DAO data = new DAO();
        data.connect();
        String sql = "INSERT INTO jadwal VALUES (" + Jadwal.getIdPemeriksa() + "," + Pasien.getRm() + "," + Dokter.getIdDokter() + "," + Suster.getIdSuster() + "," + this.idPelayan + "," + pasien.getStatus() + ");";

        if (pasien.getStatus()){
            jadwal.setPasien(pasien);
            jadwal.setDokter(dokter);
            jadwal.setSuster(suster);
            jadwal.setStatusDaftar(true);

        }
    }

    public Pasien registrasi(){
        Scanner input = new Scanner(System.in);
        DAO data = new DAO();
        data.connect();

        System.out.print("Masukkan rm baru: ");
        String newrm = input.nextLine();
        System.out.print("Masukkan nama anda : ");
        String newnama = input.nextLine();
        System.out.print("Masukkan usia anda : ");
        String newusia = input.nextLine();
        System.out.print("Masukkan alamat anda : ");
        String newalamat = input.nextLine();
        System.out.print("Masukkan penyakit anda : ");
        String newpenyakit = input.nextLine();

        Pasien newPasien = new Pasien(newnama,Integer.parseInt(newusia),newalamat);
        data.inputPasien(newPasien);


        return newPasien;
    }

    public Pelayanan(String nama){
        this.nama = nama;
    }
}
SUSTER-----------------------------------
package com.ug14.rumahsakit;

public class Suster {
    private static int idSuster = 0;
    private String nama;

    public void screening(Pasien pasien, Jadwal jadwal){
        if (jadwal.getPasien() == pasien){
            jadwal.setStatusScreening(true);
        }else{
            System.out.println("ANDA HARUS MELAKUKAN PROSES PENDAFTARAN TERLEBIH DAHULU DI BAGIAN PELAYANAN");
        }
    }

    public Suster(String nama){
        this.nama = nama;
    }

    public static int getIdSuster() {
        return idSuster;
    }
}
