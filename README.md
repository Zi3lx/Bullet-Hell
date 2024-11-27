# BulletHell 2D: BoomBoom

**Opis:**

"Strzelanka 2D: Walka z Wrogami" to dynamiczna gra akcji, w której gracz steruje bohaterem, który musi stawić czoła falom wrogów, zdobywając punkty i monety, a jednocześnie unikać wystrzeliwane przez wrogów pociki. Gra skupia się na prostych mechanikach: sterowanie postacią, strzelanie do wrogów oraz zarządzanie zasobami.

---

## Mechanika:

- **Gracz** steruje postacią, która porusza się po ekranie i strzela w kierunku kursora myszy.
- **Rodzaje wrogów**:
  - **Trójkątni wrogowie**: Mają 1 HP * poziom gry i poruszają się w stronę gracza, zadając obrażenia przy kontakcie.
  - **Wrogowie sześciokątni**: Mają 3 HP * poziom gry, strzelają pociskami, które zadają obrażenia.
  - **Bossowie**: Mają 100 HP * poziom gry, stosują różne wzory strzałów i dają dużo punktów.
- **Ulepszania**: Gracz zdobywa punkty i monety, które może wydać na ulepszenia w sklepie, takie jak zwiększenie zdrowia, prędkości czy obrażeń.
- **Zakończenie gry**: Gra kończy się, gdy HP gracza spadnie do 0.

---

## Podstawowe funkcje:

- **Sterowanie**:
  - **WASD** do poruszania się
  - **Spacja** do strzelania
- **Zbieranie punktów i monet**: Każdy typ wroga daje inne nagrody za zabicie.
- **Sklep**: Gracz może zbierać monety i kupować ulepszenia w trzech kategoriach:
  - **Zdrowie**
  - **Szybkość**
  - **Obrażenia**

---

## Cel gry:

Przetrwać jak najdłużej, pokonując wrogów, unikając obrażeń i zdobywając monety oraz punkty. Gracz stawia czoła różnym rodzajom wrogów, w tym potężnym bossom, którzy stanowią wielkie wyzwanie.

---

## Technologie:

- **Rust** – Język programowania użyty do stworzenia gry.
- **ggez** – Biblioteka do tworzenia gier 2D w Rust.
- **Nalgebra** – Biblioteka do obliczeń związanych z pozycjonowaniem i ruchem postaci oraz wrogów.

---
