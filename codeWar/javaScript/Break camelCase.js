function solution(string) {
    return string.split(/(?=[A-Z])/g).join(" ");
}