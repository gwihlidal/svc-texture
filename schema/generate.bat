"./flatc.exe" --cpp --rust --force-empty svc_texture.fbs
cp ./svc_texture_generated.rs ../src/bin/client/generated.rs