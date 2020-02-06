cd target/release/
lestech-scraper.exe > index.html
wsl scp -P 1234 index.html root@srv03.mikr.us:/var/www/html/lestech/
cd ../..