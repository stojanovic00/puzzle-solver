# Puzla projekat

Algoritam koji je korišten nema specifičan naziv, ali funkcioniše na sledeći način, na osnovu dimenzija slike određuje se veličina prozora i njegov korak kojim će on "klizati" po rešenoj slici.
Na svakom koraku pikseli koji se nalaze unutar prozora porede se sa svim učitanim delovima puzle (pore]enjem pojedina;nih RGB kanala svakog piksela u slikama) i pronalazi se najbolji kandidat, kome se upisuju kooordinate početka prozora i na taj način dodeljuje njegova pozicija na slici. Budući da u prvom prolazu neke puzle ne mogu da se reše, još jedan deo
algoritma se stara o tome da neraspodeljene delove puzle upari sa koordinatama delova gotove slike koje još uvek nikome nisu dodeljene. Ivični slučajevi su delovi koji se nalaze na obodima slike zbog toga što oni nisu jednakih dimenzija kao ostali delovi, pa su za poslednju kolonu i red
primenjivana malo drugačija pravila i ti delovi puzle su rešavani u zasebnim petljama. Svaka petlja koja rešava puzlu paralelizovana je tako da se kreira onoliko niti koliko ima procesorskih jezgara u računaru i zatim svakoj niti dodeli jednak deo puzle koji treba da proverava, a u slučaju da se
puzla ne može tačno podeliti, ostatak se ravnomerno raspoređuje na svim nitima. Sistemske niti su izabrane uprkos postojanju implementacija zelenih niti unutar određenih biblioteka iz razloga što je procesiranje u ovom algoritmu isključivo CPU-bound te se kreiranje zelenih niti i vršenje time
slicing-a
 na nivou sistemske niti ne isplati i može samo pogoršati performanse. Nakon poređenja sa sekvencijalnim rešenjem nad primerima koji su dati zajedno sa specifikacijom projekta, paralelno rešenje se ispostavilo 38% brže. 

