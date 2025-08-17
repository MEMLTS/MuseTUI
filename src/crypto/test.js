import pkg from "crypto-js";
import NodeRSA from 'node-rsa';

const { enc, AES, mode: _mode } = pkg;

function randomString(len) {
  const chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  let str = "";
  for (let i = 0; i < len; i++) {
    str += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return str;
}

// AES 加密
function aesEncrypt(data, key) {
  const keyUtf8 = enc.Utf8.parse(key);
  const iv = enc.Utf8.parse("0102030405060708");
  const encrypted = AES.encrypt(enc.Utf8.parse(data), keyUtf8, {
    iv,
    mode: _mode.CBC,
  });
  return encrypted.toString();
}

// RSA 加密
function rsaEncrypt(randomKey, pubExpHex, modulusHex) {
  const pubKey = new NodeRSA();
  pubKey.importKey({
    n: Buffer.from(modulusHex, 'hex'),
    e: parseInt(pubExpHex, 16)
  }, 'components-public');
  return pubKey.encrypt(randomKey, 'hex');
}

// 加密函数
function encrypt(data, pubExp, modulus, secKey) {
  // 第一次 AES
  const first = aesEncrypt(data, secKey);
  // 第二次 AES + 随机 key
  const randomKey = randomString(16);
  const second = aesEncrypt(first, randomKey);
  // RSA 加密 + 随机 key
  const encSecKey = rsaEncrypt(randomKey, pubExp, modulus);

  return {
    encText: second,
    encSecKey,
  };
}

export async function sign(data) {
  return encrypt(
    JSON.stringify(data),
    "010001",
    "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7",
    "0CoJUm6Qyw8W8jud"
  );
}

sign("2165763719").then(console.log);
