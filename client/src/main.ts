import "./style.css";
import * as THREE from "three";

const canvas = document.querySelector("canvas.webgl");

const scene = new THREE.Scene();

const square = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshBasicMaterial({ color: 0xff00000 });
const mesh = new THREE.Mesh(square, material);
scene.add(mesh);

const sizes = new Map();
sizes.set("x", window.innerWidth);
sizes.set("y", window.innerHeight);

const camera = new THREE.PerspectiveCamera(90, sizes.get("x") / sizes.get("y"));
camera.position.z = 3;

const renderer = new THREE.WebGLRenderer({
  canvas: canvas as Element,
});
renderer.setSize(sizes.get("x"), sizes.get("y"));
renderer.render(scene, camera);

window.addEventListener("resize", () => {
  sizes.set("x", window.innerHeight);
  sizes.set("y", window.innerHeight);

  renderer.setSize(sizes.get("x"), sizes.get("y"));
});
