Napisz program w wybranym języku programowania, który generuje parę kluczy RSA na podstawie podanych liczb pierwszych `p` i `q`. Za pomocą tego programu zasymuluj sytuację współdzielenia tego samego modułu `n` przez dwie różne osoby. Wygeneruj dwie różne pary kluczy `(sk`<sub>A</sub>`, pk`<sub>A</sub>`) = ((n, d`<sub>A</sub>`),(n, e`<sub>A</sub>`))` oraz `(sk`<sub>B</sub>`, pk`<sub>B</sub>`) = ((n, d`<sub>B</sub>`),(n, e`<sub>B</sub>`))` dla tych samych parametrów wejściowych, czyli dwóch różnych dużych liczb pierwszych `p` i `q` (pamiętaj o sprawdzaniu pierwszości parametrów wejściowych `p` oraz `q`). Następnie zakładając, że masz dostęp tylko do pary kluczy `(sk`<sub>A</sub>`, pk`<sub>A</sub>`)` oraz klucza publicznego `pk`<sub>B</sub>, zaimplementuj algorytm (podany na wykładzie) pozwalający wyliczyć klucz prywatny `sk`<sub>B</sub>.
	Rozwiązanie Znajduje się w folderze o nazwie zad1