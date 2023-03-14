use crate::{Admin, Client};
use std::io;
use crate::check_choise;
pub fn entrance(entrance1:String, admin1: Admin, admin2: Admin, client2: Client, client3: Client,
client4: Client, client5: Client){
println!("What is your name?");
let mut entrance: String = String::new();
io::stdin().read_line(&mut entrance).unwrap();
let entrance:String = entrance.trim().parse().unwrap();
// запрос строки
if entrance == "Blesk" {
println!("Welcome, {}", admin1.name);
println!("huo");
}
else if entrance == "Sajaba" {
println!("Welcome, {}", admin2.name);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Добавление товара в каталог и изменение количества товара: Редактор корзины + \n Удаление товара из каталога: Редактор каталога \n Изменить цену товара: Редактор цен \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Timur" {
println!("Welcome, {}", client2.name);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Max" {
println!("Welcome, {}", client3.name);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины  \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Artem" {
println!("Welcome, {}", client4.name);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины \n Exit - выход ");
}
else if entrance == "DeadInside" {
println!("Welcome, {}", client5.name);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Если же Вам нужна будет психологическая помощь наших операторов, то напишите: Псих.помощь \n Exit - выход ");
}

}

pub fn balance(){
let mut balance: String = String::new();
io::stdin().read_line(&mut balance).unwrap();
let balance:String = balance.trim().parse().unwrap();
if balance == "3" {
println!("Введите на какую сумму хотите пополнить ваш баланс");
let mut new_balance: String = String::new();
io::stdin().read_line(&mut new_balance).unwrap();
println!("Ваш баланс равен: {}", new_balance);
}
}

pub fn exit(){
let mut exit: String = String::new();
io::stdin().read_line(&mut exit).unwrap();
let exit:String = exit.trim().parse().unwrap();

if exit == "Exit" {
println!("До свидания, с уважением!");
}
}
pub fn psih_help(){
    println!("Мы не удивлены, что вы обратились за помощью к нашему психологу. Оцените своё состаяние от ");
    let mut psihh = String::new();
    io::stdin().read_line(&mut psihh).unwrap();
    while psihh.trim() != "Псих.помощь" {
        if psihh.trim() == "Плохо" {

        }
    }
}
pub fn shop_help(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,spisok:Vec<String>,quantitys:Vec<u64>, korzina:Vec<String>){
    println!("Мы удивлены, что вы обратили к нам за помощью касательно нашей торговой точки. \n Мы в вас разочарованы. Ладно уж, что вы хотите узнать. Выберите категорию вопроса: ");
    println!("Где я? - 100 бонусов|Как я сюда попал? - 200 бонусов|Ты глупый или что-то? - 300 бонусов");
    let mut answers_help = String::new();
    io::stdin().read_line(&mut answers_help).unwrap();
    if answers_help.trim() == "Где я?"{
        println!("Вы в магазине AQ, тут есть много чего интересного. Даже психологическая помощь, вот так то");
    }
    else if answers_help.trim() == "Как я сюда попал?" {
        println!("Мы сами не знаем как сюда попали, а вы просто запустили наш код");
    }
    else if answers_help.trim() == "Ты глупый или что-то?" {
        println!("Наверное, может быть, не знаю...");
    }
    else {
        println!("Я не знаю ответа на этот вопрос");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
    }
}