'use strict'

/**
 * Cognito CustomEmailSender trigger.
 * Decrypts the OTP with AWS Encryption SDK + KMS, logs it for E2E (B2),
 * and delivers the email via SES (Cognito no longer sends mail itself).
 */
const { buildClient, CommitmentPolicy, KmsKeyringNode } = require('@aws-crypto/client-node')
const { SESClient, SendEmailCommand } = require('@aws-sdk/client-ses')

const { decrypt } = buildClient(CommitmentPolicy.REQUIRE_ENCRYPT_ALLOW_DECRYPT)
const ses = new SESClient({})

const keyArn = process.env.KMS_KEY_ARN
const fromEmail = process.env.FROM_EMAIL

async function decryptCode(encryptedCode) {
  if (!keyArn) {
    throw new Error('KMS_KEY_ARN is required')
  }
  // Match AWS Cognito sample: generatorKeyId + keyIds both set to the CMK ARN.
  const keyring = new KmsKeyringNode({ generatorKeyId: keyArn, keyIds: [keyArn] })
  const { plaintext } = await decrypt(keyring, Buffer.from(encryptedCode, 'base64'))
  return Buffer.from(plaintext).toString('utf8')
}

exports.handler = async (event) => {
  const email = event.request.userAttributes?.email ?? ''
  const encryptedCode = event.request.code
  if (!encryptedCode) {
    throw new Error(`Missing encrypted code for trigger ${event.triggerSource}`)
  }

  const code = await decryptCode(encryptedCode)

  console.log(
    JSON.stringify({
      type: 'cognito.otp',
      trigger: event.triggerSource,
      email,
      code,
      userName: event.userName,
    }),
  )

  if (!fromEmail) {
    throw new Error('FROM_EMAIL is required')
  }
  if (!email) {
    throw new Error('Recipient email is missing')
  }

  await ses.send(
    new SendEmailCommand({
      Source: fromEmail,
      Destination: { ToAddresses: [email] },
      Message: {
        Subject: { Data: 'Your verification code', Charset: 'UTF-8' },
        Body: {
          Text: {
            Data: `Your verification code is ${code}.`,
            Charset: 'UTF-8',
          },
        },
      },
    }),
  )

  return event
}
