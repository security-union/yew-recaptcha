
# Yew-recaptcha-v3

reCAPTCHA v3 returns a score for each request without user friction. The score is based on interactions with your site and enables you to take an appropriate action for your site. Register reCAPTCHA v3 keys on the [reCAPTCHA Admin console](https://www.google.com/recaptcha/admin/create). 

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

## Analytics


```
curl -d -X -POST --header "Content-type:application/x-www-form-urlencoded" "https://www.google.com/recaptcha/api/siteverify?secret=6Ldlq3whAAAAAADSEMgRw9fNBxKn_4CJPhVwjcNq&response=03ANYolqtHdx-sKT04n7PWTVVzusYybZCNGM2evf1tkupAs5lPK6wIw2W6OjoEfSOfBoGK8dOiCp7IXvZwp3cnVXP6bAQzRko0Jt37KWzKdTRX5bosGvW9ahVRMG5sVRKJUhiER8JoWLmOZexG6ctpBM0AhC0gdwLj4V1_F47N_pEVXVergWjLYJ5Wmz7P1V8FutqY4FpSLZ_Q-KPDo030_OuvL_0We5_qTqcV7sFIW8xbVvESTRwKgJrO4z8ZPWo1I0ytX0mkXH0mpySQNYlmq7uJzVA1YX6mM_FDZs9zyzZQuSiTMnZJ9ZyruONuxXAoXvgKuuqqse4VVfw1lyUJx0uRUpVR8JGQSMsacOV2wXyDk7OGhvHVKPd1zpXZiqBAWMVHU21JJcBgAYcgtPVpaUN-Ci9z2Hbi3Uld6mwiX_aCbRYwnjqptb-VRPfyh-JkEORq_bns8XUccNWKcyRVxNkm8ppoNnx9TCboLYH0d2KB3w7QctKwCIVXKp85YwBXHCUAvEUjz91r"    
```