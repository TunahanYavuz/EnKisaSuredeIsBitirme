# Makine-İş Atama Simülasyonu (Rust)

Bu proje, farklı iş tamamlama süresi ve bozulma ihtimaline sahip makinelerle, belirli sayıda işi mümkün olan en kısa sürede tamamlamayı hedefleyen **bir iş atama simülasyonudur**.

## 📌 Problem Tanımı

- `M` adet makine vardır. Her makinenin:
  - `tamamlama_suresi`: Her bir işi kaç saniyede tamamladığı,
  - `bozulma_ihtimali`: 0 ile 1 arasında rastgele bozulma ihtimali,
  - `bozuk`: Eğer bozulmuşsa artık iş alamaz.
- `N` adet iş sırasıyla makineler arasında dağıtılır.
- Amaç, işler tamamlandığında **toplam süreyi en aza indirmek** ve **bozulmaları dikkate alarak en uygun makineyi seçmektir**.

> Eğer bir makine bozulursa, diğer makineye geçiş süresi olarak `tasima_maliyeti = 10` saniye eklenir.

## ⚙️ Nasıl Çalışır?

1. Kullanıcı, makinelerin `tamamlama_suresi` ve `bozulma_ihtimali` değerlerini girer.
2. İş sayısı girilir.
3. Program:
   - Bozuk olmayan makineler arasından işi en erken bitirecek olanı seçer.
   - Rastgele bir sayı ile bozulup bozulmadığı kontrol eder.
   - Bozulan makineler sonraki adımlarda görmezden gelir.
4. Tüm işler atanınca veya makinelerin tamamı bozulunca işlem tamamlanır.
