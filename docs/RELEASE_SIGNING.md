# Universal App Signing & Release Guide

> **Dioxus Cross-Platform Starter Release Engineering**
> Covers digital signing, certificate generation, keystore management, CI/CD secrets, and verification across **Android, iOS, macOS, Windows, and Linux**.

---

## 1. Overview of Release Signing Architecture

In production releases, every native operating system enforces mandatory code signing to:
1. Guarantee binary integrity (detect tampering and corruption).
2. Establish cryptographic proof of developer identity.
3. Satisfy OS security gates (Android Play Protect, Apple Gatekeeper, Windows SmartScreen).

```
                      [ CI / CD Release Pipeline ]
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
    [ Android ]                [ Apple ]              [ Windows ]
  release.keystore         Apple Distribution        EV Code Signing
    (RSA 2048+)             Cert + Provisioning         (.pfx / HSM)
         │                        │                        │
         ▼                        ▼                        ▼
     apksigner                 codesign                 signtool
         │                        │                        │
         ▼                        ▼                        ▼
   Signed APK/AAB           Signed .app / IPA         Signed .exe / .msi
```

---

## 2. Android Release Signing

Android requires all APKs and AABs (Android App Bundles) to be signed using **v2, v3, or v4 APK signature schemes** via `apksigner`.

### A. Generating a Production Keystore
Run `keytool` (included with OpenJDK) to generate an RSA 2048-bit key valid for 25+ years (10,000 days):

```bash
keytool -genkeypair \
  -v \
  -keystore release.keystore \
  -alias starter \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -storetype JKS
```

You will be prompted to enter:
- Keystore password (keep this confidential and backed up)
- Developer/Company Name, Organization, Country Code
- Key password (can match keystore password)

> [!CAUTION]
> Never commit `release.keystore` to version control. Store it in a secure password manager or GitHub Actions Secrets (Base64-encoded).

### B. Building and Signing via Makefile
The starter Makefile includes built-in targets for building and signing release APKs:

```bash
# 1. Build the release APK
make build-android

# 2. Sign the APK using apksigner
make sign-android KEYSTORE_PATH=release.keystore KEY_ALIAS=starter
```

### C. Manual CLI Signing Steps
If executing outside of the Makefile:

```bash
# Step 1: Align unaligned APK (4-byte alignment)
zipalign -v -p 4 \
  target/dx/starter-mobile/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk \
  target/release-aligned.apk

# Step 2: Sign with apksigner (enables v2/v3 signing schemes)
apksigner sign \
  --ks release.keystore \
  --ks-key-alias starter \
  --out target/app-signed.apk \
  target/release-aligned.apk

# Step 3: Verify the signature
apksigner verify --verbose target/app-signed.apk
```

Expected output:
```text
Verifies
Verified using v1 scheme (JAR signing): true
Verified using v2 scheme (APK Signature Scheme v2): true
Verified using v3 scheme (APK Signature Scheme v3): true
Number of signers: 1
```

### D. Google Play Store (AAB & Play App Signing)
For Play Store submission, build an Android App Bundle (`.aab`):
1. Sign the `.aab` with your **Upload Key**.
2. Upload to Google Play Console.
3. Google Play App Signing strips the upload signature and re-signs the delivered APKs with Google's managed private master key.

---

## 3. Apple iOS & iPadOS Release Signing

iOS requires code signing via an **Apple Developer Account** ($99/year).

### A. Required Credentials
1. **Apple Distribution Certificate** (`.p12` file installed in macOS Keychain).
2. **Bundle Identifier**: Must match `Dioxus.toml` (`dev.dioxus.starter`).
3. **App Store Distribution Provisioning Profile** (`.mobileprovision`).

### B. Automated Signing via Xcode / Dioxus CLI
When running on macOS:
```bash
# Build the iOS bundle in release mode
make build-ios
```
Configure `project.pbxproj` or pass signing identities:
```bash
xcodebuild -workspace ... \
  -scheme starter-mobile \
  -configuration Release \
  -archivePath build/starter.xcarchive \
  archive \
  CODE_SIGN_STYLE="Manual" \
  CODE_SIGN_IDENTITY="Apple Distribution: Your Team (TEAM_ID)" \
  PROVISIONING_PROFILE_SPECIFIER="Your_Profile_Name"
```

### C. Exporting & Submitting
```bash
# Export IPA for App Store submission
xcodebuild -exportArchive \
  -archivePath build/starter.xcarchive \
  -exportOptionsPlist exportOptions.plist \
  -exportPath build/ipa

# Validate and upload using xcrun notarytool / altool
xcrun altool --upload-app \
  --type ios \
  --file build/ipa/starter-mobile.ipa \
  --apiKey YOUR_API_KEY \
  --apiIssuer YOUR_ISSUER_UUID
```

---

## 4. macOS Desktop Signing & Notarization

macOS Gatekeeper blocks unsigned applications and requires **Notarization** by Apple's automated scanner.

### A. Signing the `.app` Bundle
Use a **Developer ID Application** certificate with Hardened Runtime enabled:

```bash
codesign --deep --force --verify --verbose \
  --options runtime \
  --sign "Developer ID Application: Company Name (TEAM_ID)" \
  --entitlements entitlements.plist \
  target/dx/starter-desktop/release/macos/app.app
```

### B. Notarizing with Apple
```bash
# Zip the signed app
ditto -c -k --keepParent target/dx/starter-desktop/release/macos/app.app starter-mac.zip

# Submit to Apple Notary Service
xcrun notarytool submit starter-mac.zip \
  --apple-id "developer@company.com" \
  --team-id "TEAM_ID" \
  --password "app-specific-password" \
  --wait

# Staple notarization ticket to the app bundle
xcrun stapler staple target/dx/starter-desktop/release/macos/app.app
```

Once stapled, users can open the app on macOS without Gatekeeper security warnings.

---

## 5. Windows Desktop Signing (Authenticode)

Windows SmartScreen warns users against running unknown `.exe` files unless signed with an **Authenticode Certificate** (Standard OV or Extended Validation EV).

### A. Signing with `signtool.exe`
```cmd
signtool sign ^
  /tr http://timestamp.digicert.com ^
  /td sha256 ^
  /fd sha256 ^
  /a ^
  /f "credentials\cert.pfx" ^
  /p "%CERT_PASSWORD%" ^
  target\dx\starter-desktop\release\windows\starter-desktop.exe
```

### B. Verifying Signature
```cmd
signtool verify /pa /v target\dx\starter-desktop\release\windows\starter-desktop.exe
```

---

## 6. Linux Packaging & GPG Signing

For Linux distributions:
- **Debian/Ubuntu (`.deb`)**:
  ```bash
  dpkg-sig --sign builder target/dx/starter-desktop/release/linux/starter-desktop.deb
  ```
- **AppImage**:
  ```bash
  appimagetool --sign --sign-key YOUR_GPG_KEY_ID AppDir starter-x86_64.AppImage
  ```

---

## 7. CI/CD Automated Secrets Management (GitHub Actions)

To automate signing in GitHub Actions without exposing secrets in repositories:

| Secret Name | Platform | Description |
| :--- | :--- | :--- |
| `ANDROID_KEYSTORE_BASE64` | Android | `base64 -w 0 release.keystore` |
| `ANDROID_KEYSTORE_PASSWORD`| Android | Password for the keystore file |
| `ANDROID_KEY_ALIAS` | Android | Key alias (e.g. `starter`) |
| `ANDROID_KEY_PASSWORD` | Android | Password for the private key |
| `APPLE_CERT_P12_BASE64` | macOS / iOS | Base64-encoded Apple `.p12` certificate |
| `APPLE_CERT_PASSWORD` | macOS / iOS | Password for `.p12` certificate |
| `APPLE_PROVISIONING_PROFILE`| iOS | Base64-encoded `.mobileprovision` file |
| `WIN_CERT_BASE64` | Windows | Base64-encoded `.pfx` Authenticode certificate |
| `WIN_CERT_PASSWORD` | Windows | Password for `.pfx` certificate |

### Example GitHub Actions Step for Android:
```yaml
- name: Decode & Sign Android APK
  env:
    KEYSTORE_BASE64: ${{ secrets.ANDROID_KEYSTORE_BASE64 }}
    KEYSTORE_PASS: ${{ secrets.ANDROID_KEYSTORE_PASSWORD }}
    KEY_ALIAS: ${{ secrets.ANDROID_KEY_ALIAS }}
    KEY_PASS: ${{ secrets.ANDROID_KEY_PASSWORD }}
  run: |
    echo "$KEYSTORE_BASE64" | base64 --decode > release.keystore
    make sign-android KEYSTORE_PATH=release.keystore KEYSTORE_PASSWORD="$KEYSTORE_PASS" KEY_ALIAS="$KEY_ALIAS" KEY_PASSWORD="$KEY_PASS"
```
