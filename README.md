# Project Mademoiselle Entropia [ UV ].
# GOAL:

Extremely strong; maximally simple and relatively fast symmetric cipher.
# How it works..
First key(Prime Key) is multiuse from one file to another; second (Included Key) is created for every time with  generator of random numbers (W-Chaos in 1st version & Order of the Chaos for new ones); cipher uses IK  to encrypt a file, & uses PK to inject IK (ciphered with PK) into cryptotext.  by operations, it works so:

1. to get IK from generator;
2. to cipher file with IK;
3. to cipher IK with PK;
4. to inject/insert encrypted IK into cryptotext.

# How to use..
  1. Build TAM with MAE: cargo build --features=mae.
  2. Within Console: sark0y_tam_rst -encrypt-copy /path/to/file; sark0y_tam_rst -decrypt-copy /path/to/file.mae
  3. Within TAM: encrypt copy /path/to/file. Remark: use sub-command {Insert}{no esc} to feed program with correct path.
  4. The way to integrate it in Your Project: https://github.com/SarK0Y/TAM_RUSTy/blob/pre-workable/src/mae.rs

 # Links: <br>
 <b>TELEGRAM:</b> https://t.me/+N_TdOq7Ui2ZiOTM6 (Alg0Z). <br>
 <b>ALG0Z RU:</b> https://dzen.ru/alg0z <br>
 <b>ALG0Z EN:</b> https://alg0z.blogspot.com <br>
 <b>ChangeLog:</b> https://alg0z8n8its9lovely6tricks.blogspot.com/2023/09/tam-changelog.html <br>
 <b>FORUM:</b> https://www.neowin.net/forum/topic/1430114-tam/ <br>
 <b>E-MAIL:</b> sark0y@protonmail.com <br>
 <b>MAE's sources</b> https://github.com/SarK0Y/Mademoiselle_Entropia <br>
 <b>Automation Manager with MAE:</b> https://github.com/SarK0Y/TAM_RUSTy.git <br>
 <b>YouTube:</b> https://www.youtube.com/@evgeneyknyazhev968 <br>
 <b>Twitter_X:</b> https://x.com/SarK0Y8 <br>
 Donations: https://boosty.to/alg0z/donate <br>
 
# my the Best Wishes to You 🙃
