package com.ug9.mobilelegend;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        Character infantry = new Minion("Infantry",30,150);
        Character lancer = new Minion("Lancer",40,250);
        Character canon = new Minion("Canon",50,350);
        Character fiend = new Creep("Fiend",40,350);
        Character serpent = new Creep("Serpent",50,350);
        Character crab = new Creep("Crab",50,400);
        Character fanny = new Melee("Fanny",550,1200);
        Character alucard = new Melee("Alucard",400,1500);
        Character layla = new Ranged("Layla",600,900);
        Character eudora = new Ranged("Eudora",700,800);
        System.out.println("----------Skenario 1: Minion menyerang Creep----------");
        infantry.attack(serpent);
        System.out.println("----------Skenario 2: Creap menyerang Karakter NPC----------");
        serpent.attack(lancer);
        serpent.attack(fiend);
        System.out.println("----------Skenario 3: Minion menyerang Minion----------");
        infantry.attack(lancer);
        System.out.println("----------Skenario 4: Minion menyerang Hero----------");
        canon.attack(alucard);
        System.out.println("----------Skenario 5: Hero menyerang Hero----------");
        fanny.attack(layla);
        ((Ranged) eudora).ultimate();
        eudora.attack(fanny);
        System.out.println("----------Skenario 6: Hero membunuh Minion----------");
        layla.attack(infantry);
        fanny.attack(canon);
        System.out.println("----------Skenario 7: Hero membunuh Creep----------");
        layla.attack(serpent);
        fanny.attack(fiend);
        System.out.println("----------Skenario 8: Hero membunuh Hero----------");
        ((Melee) fanny).ultimate();
        fanny.attack(layla);
        System.out.println("----------Skenario 9: Hero Max Health----------");
        alucard.attack(crab);
        System.out.println("----------Skenario 10: Informasi Karakter----------");
        printDetailCharacter(infantry);
        printDetailCharacter(lancer);
        printDetailCharacter(canon);
        printDetailCharacter(fiend);
        printDetailCharacter(serpent);
        printDetailCharacter(crab);
        printDetailCharacter(fanny);
        printDetailCharacter(layla);
        printDetailCharacter(eudora);
        printDetailCharacter(alucard);
    }
    public static void printDetailCharacter(Character ch){
        System.out.println("=====Detail Karakter=====");
        System.out.println("Nama\t\t: "+ch.getName());
        if (ch instanceof Ranged || ch instanceof Melee){
            System.out.println("Tipe\t\t: Hero");
            System.out.println("Level\t\t: "+((Hero) ch).getLevel());
        } else if (ch instanceof Minion || ch instanceof Creep){
            System.out.println("Tipe\t\t: NPC");
        }
        System.out.println("---> Health\t: "+ch.getHealth());
    }
}
