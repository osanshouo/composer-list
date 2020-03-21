export function execCopy(text) {
    // https://qiita.com/simiraaaa/items/2e7478d72f365aa48356
    var tmp = document.createElement("input");
    tmp.type = "text";
    tmp.value = text;
  
    // 要素を画面外へ
    var s = tmp.style;
    s.position = 'fixed';
    s.right = '200%';
  
    // body に追加
    document.body.appendChild(tmp);

    // 要素を選択
    tmp.select();
  
    // クリップボードにコピー
    var result = document.execCommand("copy");
  
    // 要素削除
    document.body.removeChild(tmp);
  
    // return result;
    let input = document.querySelector("input")
    input.value = "";
    input.focus();
}
