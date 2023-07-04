# kindle highlight

```bash
docker run -d -p 4444:4444 -p 7900:7900 --shm-size="2g" selenium/standalone-chrome
```

WebDriver http://localhost:4444

(Optional) To see what is happening inside the container, head to http://localhost:7900/?autoconnect=1&resize=scale&password=secret.