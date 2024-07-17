<script>
    export let icon;
    export let label;
    export let func;
  </script>

  <div class="d-flex flex-column align-items-center">
      <button class="btn-square" on:click={func}>
         <i class={`bi ${icon}`}></i>
      </button>
      <div class="btn-label">{label}</div>
  </div>

  <style>
    .btn-square {
        width: 100px;
        height: 100px;
        background-color: #1b1e21;
        color: #ffffff;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
        transition: transform 0.1s, box-shadow 0.1s;
        border: none;
    }
    .btn-square:hover {
        transform: scale(0.95);
        box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.4);
    }
    .btn-square i {
        font-size: 4rem;
    }
    .btn-label {
        margin-top: 10px;
        text-align: center;
        color: #ffffff;
    }
  </style>
