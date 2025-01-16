# Otoczony

**Opis:**

"Otoczony" to gra typu BulletHell, w której gracz steruje bohaterem, który musi stawić czoła falom wrogów, zdobywając punkty i monety, a jednocześnie unikając obrażeń. Gra skupia się na prostych mechanikach: sterowanie postacią, strzelanie do wrogów oraz zarządzanie zasobami.

---

## Mechanika:

- **Gracz** steruje postacią, która porusza się po ekranie i strzela w kierunku kursora myszy.
- **Rodzaje wrogów**:
  - **Wrogowie Bomby**: Są małymi niestrzelającymi przecinikami, którzy zadają obrażenia poprzez styczność z przeciwnikiem
  - **Wrogowie Taco**: Są wolniejszymi i wiekszymi przeciwnikami, których celem jest strzelanie do gracza
  - **Bossowie**: Stosują różne wzory strzałów i dają dużo punktów.
- **Ulepszania**: Gracz zdobywa punkty i monety, które może wydać na ulepszenia w sklepie, takie jak zwiększenie zdrowia, prędkości czy obrażeń.
- **Zakończenie gry**: Gra kończy się, gdy HP gracza spadnie do 0.

---

## Podstawowe funkcje:

- **Sterowanie**:
  - **WSAD** do poruszania się
  - **Spacja / Lewy przycisk myszy** do strzelania
- **Zbieranie punktów i monet**: Każdy typ wroga daje inne nagrody za zabicie.
- **Sklep**: Gracz może zbierać monety i kupować ulepszenia w czterech kategoriach:
  - **Zdrowie**
  - **Obrażenia**
  - **Prędkość prouszania się**
  - **Szybkość ataku**

---

## Cel gry:

Przetrwać jak najdłużej, pokonując wrogów, unikając obrażeń i zdobywając monety oraz punkty. Gracz stawia czoła różnym rodzajom wrogów, w tym bossom.

---

## Technologie:

- **Rust** – Język programowania użyty do stworzenia gry.
- **ggez** – Biblioteka do tworzenia gier 2D w Rust.
- **Nalgebra** – Biblioteka do obliczeń związanych z pozycjonowaniem i ruchem postaci oraz wrogów.
- **RNG**

---

## Uruchomienie Programu

Program był napisany na na systemi **macOS** i by uruchomć go na na tym systemie należy przejść do floderu src i wpisać cargo build / cargo run. 

Program działą również pod systemem **Linux** jednak może być cięższy do uruchomienia.


<img width="748" alt="Zrzut ekranu 2024-12-18 o 12 39 14" src="https://github.com/user-attachments/assets/b653f0e2-9b68-406d-9c10-9409e137dc3e" />

<img width="744" alt="Zrzut ekranu 2024-12-18 o 12 40 37" src="https://github.com/user-attachments/assets/d618cf43-8776-42ca-aa07-98a0a6f12019" />





