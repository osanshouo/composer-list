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
    document.execCommand("copy");
  
    // 要素削除
    document.body.removeChild(tmp);
}

export function execCopyTarget(target) {
    let text = target.textContent;

    console.log(target.textContent);

    // copy
    execCopy(text);

    // animation
    target.classList.add("effect");
    setTimeout(() => {
        target.classList.remove("effect");
    }, 500);

    let input = document.querySelector("input")
    input.focus();
};
