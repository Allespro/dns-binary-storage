# DNS Binary Storage

This tool allows you to store binary files in your domain's DNS records and retrieve them using DNS-over-HTTPS (DoH).

## How It Works

1. When encoding a file:
   - The file is compressed using zstd
   - Compressed data is encoded in base64
   - DNS TXT records are created with fragments of the encoded data
   - Each record has a prefix with a sequence number for proper file reconstruction

2. When extracting a file:
   - Data is requested via DoH (DNS-over-HTTPS)
   - Records are assembled in the correct order
   - Data is decoded from base64
   - zstd decompression is performed
   - The recovered file is saved

## Usage

### Encoding a File into DNS Records

```bash
dns-binary-storage to-records --domain your.domain.com --input-path ./your-file.jpg --output-path ./dns-records.txt
```

This command will create a `dns-records.txt` file containing DNS records in the format:
```
your.domain.com.    3600    IN    TXT    "0:base64data..."
your.domain.com.    3600    IN    TXT    "1:base64data..."
your.domain.com.    3600    IN    TXT    "2:base64data..."
```

You can upload these records to your DNS zone (for example, via the Cloudflare control panel).

### Extracting a File from DNS

```bash
dns-binary-storage doh --domain your.domain.com --output-path ./recovered-file.jpg
```

This command will:
1. Request all TXT records for the domain via Cloudflare DoH
2. Assemble and decode the data
3. Save the recovered file to the specified path