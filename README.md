Projekt GierkiRustowe

W projekcie skupiłem się na stworzeniu aplikacji, która posiada responsywne UI zarówno na kliknięcia myszką jak i użycie klawiatury. Zawarłem również obsługę pliku oraz stworzenie widoków za pomocą paczki GGEZ.
Projekt wymaga rustc conajmniej w wersji 1.81
Po uruchomieniu przenosimy się do menu głównego, w którym znajdują sie przyciski przejścia do gier oraz tabela najlepszych wyników.
Flappy bird - gra w której musimy skakać ptakiem (w naszym przypadku kwadratem) pomiędzy rurami, unikając zderzeć, skok wykonuje się spacją
Snake - gra, której chyba nie trzeba przedstawiać. Plansza ma niestandardowe wymiary (chodzi mi o liczbę kwadratów), ponieważ dostosowałem ją do rozmiaru okna.
Krawędzie w obydwu grach są nieprzechodne, a wlecenie w nie ptakiem, bądź wejście wężem skutkuje śmiercią i przegraniem.

Obie gry zliczają punkty w czasie trwania gry, a wynik jest wyświetlany na końcu. Aplikacja w przyszłości będzie rozbudowana o kolejne mini gry, tak żeby umilić sobię chociażby drogę w pociągu,
choć wszyscy dobrze wiedzą, że snake'a przyjemniej się pisze, niż w niego gra.

testowane na:

rustc --version
rustc 1.84.0
cargo --version
cargo 1.84.0
