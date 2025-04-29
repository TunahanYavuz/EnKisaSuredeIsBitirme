use std::io;
use rand::{rng, Rng};
#[derive(Debug)]
struct Makine {
    id: i16,
    tamamlama_suresi:i32,
    bozulma_ihtimali:f32,
    bitis_suresi:i32,
    bozuk:bool
}
impl Makine {
    fn new(id: i16, tamamlama_suresi:i32, bozulma_ihtimali:f32)->Self{
        Self{id, tamamlama_suresi, bozulma_ihtimali, bozuk: false, bitis_suresi:0}
    }
}

fn main() {
    let mut id = 0;
    println!("Lütfen Makinelerin İş Tamamlama Sürelerini(saniye ve tam sayılar) ve Bozulma İhtimallerini(0-1 arası ondalıklı sayılar) Sırayla Giriniz.\n\
    Her Girdiğiniz Değer Yeni Bir Makine Oluşturacaktır.");
    let mut makineler : Vec<Makine> = vec![];
    loop {
        let mut tamamlama_suresi = String::new();
        let mut bozulma_ihtimali = String::new();
        io::stdin().read_line(&mut tamamlama_suresi).expect("Tamamlama Süresi Okunamadı");
        io::stdin().read_line(&mut bozulma_ihtimali).expect("Bozulma İhtimali Okunamadı");
        let tamamlama_trim = tamamlama_suresi.trim();
        let bozulma_trim = bozulma_ihtimali.trim();
        if tamamlama_trim.is_empty() || bozulma_trim.is_empty() {
            println!("Boş Giriş Yapılamaz!");
            continue;
        }
        let mut bozulma_ihtimali : f32;
        match bozulma_trim.parse::<f32>(){
            Ok(sayi) => {
                if sayi<0.0 || sayi>1.0 {
                    println!("0' ın altında ve 1' den büyük bir değer bozulma değeri olarak verilemez");
                    continue
                }
                bozulma_ihtimali = sayi; }
            Err(_) => {
                println!("Geçersiz Giriş! Sayısal Bir Değer Giriniz.");
                continue
            },
        }
        match tamamlama_trim.parse::<i32>(){
            Ok(sayi) => {
                if sayi<=0 {
                    println!("Tamamlama Süresi 0' dan Küçük Veya Eşit Olamaz.");
                    continue
                }
                makineler.push(Makine::new(id, sayi, bozulma_ihtimali)) ;
                id += 1;
            },
            Err(_) => {
                println!("Geçersiz Giriş! Sayısal Bir Değer Giriniz.");
                continue
            },
        }
        println!("Devam Etmek İçin 'd' Tuşuna Basınız.");
        let mut devam = String::new();
        io::stdin().read_line(&mut devam).expect("Okunamadı");
        match devam.trim() {
            "d" => continue,
            _ => break
        }
    }
    println!("\nİş Sayısını Giriniz");
    let mut is_sayisi = String::new();
    io::stdin().read_line(&mut is_sayisi).expect("Okunamadı.");
    let is_trim = is_sayisi.trim();
    let mut is_sayisi = is_trim.parse::<i32>().expect("Tam Sayı Değerine Çevrilemedi.");
    let mut tasima_maliyeti:i32 = 0;
    let mut is_id = 0;
    while is_id < is_sayisi {
        let sec_index = (0..makineler.len())
            .filter(|&i| !makineler[i].bozuk)
            .min_by_key(|&i| makineler[i].bitis_suresi + makineler[i].tamamlama_suresi+tasima_maliyeti);

        if let Some(i) = sec_index {
            let makine = &mut makineler[i];
            let r: f32 = rng().random_range(0.0..1.0);
            if r < makine.bozulma_ihtimali {
                makine.bozuk = true;
                tasima_maliyeti = 10;
                println!("Makine {} Bozuldu! Taşıma Maliyeti {} Eklendi.", makine.id+1, tasima_maliyeti);
                is_id-=1;
            }else {
                makine.bitis_suresi += makine.tamamlama_suresi+tasima_maliyeti;
                tasima_maliyeti = 0;
                println!(
                    "İş {} -> Makine {} (Yeni Bitiş Süresi: {})",
                    is_id + 1,
                    makine.id+1,
                    makine.bitis_suresi
                );
            }
        } else {
            println!("İş {} Atanamadı: Tüm Makineler Bozuk.", is_id + 1);
            println!("Kalan İş Sayısı: {}",is_sayisi-(is_id));
            break;
        }
        is_id += 1;
    }
    println!("\nİşlem Tamamlandı. Son Durum:");
    for makine in makineler{
        println!("Makine {} Çalışma Süresi {} Bozuk Mu {}",makine.id,makine.bitis_suresi, makine.bozuk);
    }
}
