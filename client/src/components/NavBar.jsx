import "bootstrap/dist/css/bootstrap.min.css";

const NavBar = () => {
    return(
        <div>
            <nav class="navbar navbar-expand-lg bg-body-tertiary">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#">Nombre pagina</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                          <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">Catálogo</a>
                          </li>
                          <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">Sobre nosotros</a>
                          </li>
                          <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">Tercero</a>
                          </li>
                        </ul>
                        <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
                          <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">Registrarse</a>
                          </li>
                          <li class="nav-item">
                            <a class="nav-link active" aria-current="page" href="#">Iniciar sesión</a>
                          </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </div>
    )
}

export default NavBar;