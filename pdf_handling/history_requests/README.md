This is where PDFs are written to. They are immediately deleted upon being uploaded, however.

curl -v -F fileuploadfront=@test.mp4 -F fileuploadside=@test.mp4 -F uid=curlplaceholder -F age=18 -F ethnicity=Caucasian -F email=me@hiibolt.com -F sex=M -F height="5'10" -F weight=120 http://api.igaitapp.com/api/v1/upload