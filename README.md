![Ww0Xt1mAMy31Ofar0GYu8Oab0v2k0uF1XT_zTt5kPU1M8o58sT5OOXCsSxv3nNGxsG8dG4zI=w1060-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj (1)](https://user-images.githubusercontent.com/1176339/155262320-ce1406f0-d35d-418e-a8b9-60b928cceeb2.jpeg)


![google-reCAPTCHA](https://user-images.githubusercontent.com/1176339/184780286-856b249d-01e6-498f-a9af-8486dedbdc16.svg)

**Disclaimer:** This is an unnoficial library. Google reCAPTCHA is owned by Google, this library is maintained by Security Union LLC. 

# YEW-reCAPTCHA-v3

[![crates.io](https://img.shields.io/crates/v/yew-recaptcha-v3.svg)](https://crates.io/crates/yew-recaptcha-v3)
[![docs.rs](https://docs.rs/yew-recaptcha-v3/badge.svg)](https://docs.rs/yew-recaptcha-v3)


## TLDR

reCAPTCHA v3 returns a score for each request without user friction. The score is based on interactions with your site and enables you to take an appropriate action for your site. Register reCAPTCHA v3 keys on the [reCAPTCHA Admin console](https://www.google.com/recaptcha/admin/create). 

## YouTube Tutorial
https://www.youtube.com/watch?v=qnJSexoFcmM

## How to use it?
Checkt out the [examples](/examples/) folder.

## Verifying the user's response

This page explains how to verify a user's response to a reCAPTCHA challenge from your application's backend.

Please refer to https://developers.google.com/recaptcha/docs/verify

### Token Restrictions

Each reCAPTCHA user response token is valid for two minutes, and can only be verified once to prevent replay attacks. If you need a new token, you can re-run the reCAPTCHA verification.

After you get the response token, you need to verify it within two minutes with reCAPTCHA using the following API to ensure the token is valid.

```
curl -d -X -POST --header "Content-type:application/x-www-form-urlencoded" "https://www.google.com/recaptcha/api/siteverify?secret=6Ldlq3whAAAAAADSEMgRw9fNBxKn_4CJPhVwjcNq&response=<token>"    
```

Sample response:

```
{
  "success": true,
  "challenge_ts": "2022-08-16T01:25:24Z",
  "hostname": "localhost",
  "score": 0.9,
  "action": "submit"
} 
```

### Interpreting the score

reCAPTCHA v3 returns a score (1.0 is very likely a good interaction, 0.0 is very likely a bot). Based on the score, you can take variable action in the context of your site. Every site is different, but below are some examples of how sites use the score. As in the examples below, take action behind the scenes instead of blocking traffic to better protect your site.

