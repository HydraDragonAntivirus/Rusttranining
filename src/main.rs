// https://www.youtube.com/watch?v=0t_jpl70Mbg&list=PLgvWD2scL860_6ppZQS6i86vQuX_5wV2
// Main fonksiyonunu oluşturmak istediğimiz için function main dekserasyonu oluşturuyoruz ve parantezler içerisinde de arıyorum ilk başta main fonksiyonunu arıyor birde süslü parantezleri arıyoruz
//tam olarak fonksiyon denemez kırmızı çizgimiz noktalı ve her zaman koymalıyız 
//fn main() {
 //   println!("Hello, rust!");
//} // ders 1 function fonksiyon main fonskiyonun println! ile print ediyoruz ve ;'yı unutmamak lazım
//ders 2
fn main () {
    // mut olsun ki değiştirebilirim sonradan
    let mut age = 30; // value assigned to age is never read çünkü onu 33 yaptım
    age = 33; //sonradan değiştirdim
    const PI_NUMBER: f32 = 3.14159; // pi sayısı
    println!("{} {}" ,PI_NUMBER, age); // {]} ve age ile çağırdım veya PI Number ile iki kez çağırma
} 
