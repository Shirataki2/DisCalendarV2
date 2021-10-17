---
title: ログイン
description: '001'
---

まずはこのサイトへログインし，あなたのDiscordアカウントと連携を行います．

<Btn
  outlined
  color="primary"
  :external="true"
  to="https://discord.com/api/oauth2/authorize?client_id=771795045543313409&redirect_uri=https%3A%2F%2Fdiscalendar.app%2Fcallback&response_type=code&scope=identify%20guilds">
  ログイン
</Btn>

<VImage src="/docs/login.jpg"></VImage>

どちらの権限もこのサーバーであなたに関する情報と，あなたが属しているサーバーに関する情報の取得に際してのみ使用します．

ログインが成功すると右上にご自身のアバターが表示されます．

<VImage src="/docs/avatar.png"></VImage>

ログイン成功後はダッシュボードページにアクセスするか，Botを使いたいサーバーにまだ導入していない状態でしたらBotの新規導入を行ってください．

<Btn
  outlined
  color="info"
  to="/dashboard">
  ダッシュボードページへ
</Btn>

もしログインに失敗するようでしたら，サポートサーバーの「#不具合報告」チャンネルにてその旨をお知らせください．

<Btn
  outlined
  external
  color="error"
  to="https://discord.gg/MyaZRuze23">
  サポートサーバーに参加する
</Btn>

