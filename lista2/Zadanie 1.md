Zorientuj się, jak mozna policzyć wartość funkcji hashującej od wybranego ciągu znaków. Zainstaluj Hashcat (https://hashcat.net/hashcat/) i zapoznaj się z podstawową dokumentacją (https://hashcat.net/wiki/doku.php?id=hashcat). Następnie:

1. Policz wartość funkcji haszującej dla hasła składającego się wyłącznie z cyfr, a następnie mając informację, jaką funkcję została użyta i mając wartość hasha, spróbuj złamać to hasło przy użyciu Hashcat-a w ataku typu brute-force, zakładając, że hasło składa się tylko z cyfr. Jak długie tego typu hasła można w rozsądnym czasie złamać na twojej maszynie, przy założeniu, że do hashowania używana była nieodpowiednia do tego celu funkcja haszująca (np. md5, czy też z rodziny SHA)? Z jakimi opcjami uruchamiałeś/eś Hashcat-a?

Rozwiązanie: [[1.1]]

2. Proszę powtórzć ćwiczenie powyżej, zakładając, że hasło jest kombinacją małych liter i cyfr.

Rozwiązanie: [[1.2]]

3. Pobierz wybrany słownik haseł powszechnie uznawanych za słabe. Wybierz jedno hasło z tego słownika i lekko zmodyfikuj je, dodając znak specjalny oraz kilka cyfr na końcu. Następnie oblicz wartość funkcji haszującej dla tego zmodyfikowanego hasła. Po wykonaniu obliczeń przeprowadź atak hybrydowy z wykorzystaniem słownika oraz odpowiedniej maski. Z jakimi opcjami uruchamiałeś/aś Hashcat-a?

Rozwiązanie: [[1.3]]

4. Wymieńcie się z koleżanką/kolegą wartościami funkcji haszującej dla waszych haseł. Nie ujawniajcie sobie hasła ani rodzaju funkcji haszującej, która została użyta. Spróbujcie złamać hasło koleżanki/kolegi. Jak możecie zacząć, jeśli nie wiecie, jakiej funkcji haszującej użyto? Czy można zawęzić zbiór poszukiwań?

Rozwiązanie: [[1.4]]


## Plik z wynikami z podpunktów 1-4: [[Wyniki]]