import "bootstrap/dist/css/bootstrap.min.css";

const login = () => {

  return (
      <div class="container-fluid">
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

        <div class="container-sm p-5">
          <div class="row justify-content-center align-items-center">
              <div class="col-10 text-center">
                  <p class="fw-bolder fs-2">Iniciar sesión</p>
              </div>
              <div class="col-10 col-sm-6 col-md-4 col-lg-3">
                <form>
                  <div class="mb-3">
                    <label htmlFor="rut" class="form-label text-body-secondary">Rut</label>
                    <br />
                    <input
                      type="text"
                      id="rut"
                      aria-describedby="rutHelp"
                    />
                  </div>
                  <div class="mb-3">
                    <label htmlFor="pass" class="form-label text-body-secondary">Contraseña</label>
                    <br />
                    <input
                      type="password"
                      id="pass"
                      aria-describedby="passHelp"
                    />
                  </div>
                  <div class="d-grip gap-2 d-md-block">
                    <button type="button" class="btn btn-secondary text-center form-control">Ingresar</button>
                  </div>
                </form>
              </div>
          </div>
        </div>
      </div>
  );
};

export default login;
