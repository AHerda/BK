Przeanalizuj nagłówki kilku wybranych wiadomości ze swojej skrzynki odbiorczej i kilku z folderu spam pod kątem mechanizmów uwierzytelniania wiadomości.

1. Czy serwery pocztowe, z których pochodzą wiadomości, są upoważnione do wysyłania wiadomości w imieniu domeny nadawcy? Sprawdź nie tylko status, ale także użyj narzędzia dig do weryfikacji, czy serwer pocztowy nadawcy jest na liście dozwolonych serwerów dla danej domeny.
    [[2.1]]
2. Czy wiadomości zostały podpisane cyfrowo przez domenę nadawcy, a podpis został poprawnie zweryfikowany przez serwery pocztowe? Jakie elementy zostały podpisane?
    [[2.2]]
3. Czy stosowany jest mechanizm DMARC? Jaki zwraca status?
    [[2.3]]
4. Sprawdź politykę DMARC dla domeny przy użyciu narzędzia `dig`. Jakie są konsekwencje ustawienia konkretnej polityki?
    [[2.4]]
5. Czy może się zdarzyć, że SPF, DKIM oraz DMARC zwrócą status `pass`, a pomimo to wiadomość trafi do spamu?
    [[2.5]]
6. Czy może się zdarzyć, że SPF, DKIM oraz DMARC zwrócą status `pass`, a pomimo to wiadomość zawiera złośliwy link/załącznik?
	[[2.6]]