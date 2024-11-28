# Strzelanka 2D: BoomBoom

**Opis:**

"Strzelanka 2D: BoomBoom" to gra typu BulletHell, w której gracz steruje bohaterem, który musi stawić czoła falom wrogów, zdobywając punkty i monety, a jednocześnie unikając obrażeń. Gra skupia się na prostych mechanikach: sterowanie postacią, strzelanie do wrogów oraz zarządzanie zasobami.

---

## Mechanika:

- **Gracz** steruje postacią, która porusza się po ekranie i strzela w kierunku kursora myszy.
- **Rodzaje wrogów**:
  - **Trójkątni wrogowie**: Mają 1 HP i poruszają się w stronę gracza, zadając obrażenia przy kontakcie.
  - **Wrogowie sześciokątni**: Mają 3 HP, strzelają pociskami, które zadają obrażenia.
  - **Bossowie**: Mają 100 HP, stosują różne wzory strzałów i dają dużo punktów.
- **Ulepszania**: Gracz zdobywa punkty i monety, które może wydać na ulepszenia w sklepie, takie jak zwiększenie zdrowia, prędkości czy obrażeń.
- **Zakończenie gry**: Gra kończy się, gdy HP gracza spadnie do 0.

---

## Podstawowe funkcje:

- **Sterowanie**:
  - **WASD** do poruszania się
  - **Spacja** do strzelania
- **Zbieranie punktów i monet**: Każdy typ wroga daje inne nagrody za zabicie.
- **Sklep**: Gracz może zbierać monety i kupować ulepszenia w czterech kategoriach:
  - **Zdrowie**
  - **Szybkość**
  - **Obrażenia**
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

<img width="753" alt="Zrzut ekranu 2024-11-28 o 14 48 25" src="https://github.com/user-attachments/assets/93911bfb-ced7-4b1b-b13c-ba1592d93eee">


