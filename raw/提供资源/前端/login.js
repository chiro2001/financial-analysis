(function(){
    function handleLogin() {
        // 拿到登录信息
        var userName = document.querySelector('.usrName input').value
        var userPassword = document.querySelector('.usrPwd input').value
        alert('用户'+userName+'登陆成功')
        showChatRoom()
    }
    function showChatRoom() {
        // 隐藏登录框
        var login = document.querySelector('.login')
        login.style.top = '-100%'

    }
    var loginButton = document.querySelector('.login-button')
    loginButton.addEventListener('click', function(e) {
        handleLogin()
    }, false)

})()