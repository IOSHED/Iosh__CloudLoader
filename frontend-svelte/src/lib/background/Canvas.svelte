<script>
    import { onMount } from 'svelte';

    // Константы для настроек
    const ATTRACTION_FORCE = 0.01; // Сила притяжения к углам
    const MUTUAL_ATTRACTION_FORCE = 0.005; // Сила притяжения друг к другу
    const REPULSION_FORCE = 0.05; // Сила отталкивания от центров
    const MAX_ATTRACTION_DISTANCE = 100; // Максимальная дистанция для притяжения
    const MAX_SPEED = 3; // Максимальная скорость шарика

    export let colorsBalls;
    export let quantityBalls = 10;
    export let speedBalls = 70;

    let canvas;
    let ctx;
    let balls = [];

    class Ball {
        constructor(x, y, radius, color, dx, dy) {
            this.x = x;
            this.y = y;
            this.radius = radius;
            this.color = color;
            this.dx = dx;
            this.dy = dy;
        }

        draw() {
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
            ctx.fillStyle = this.color;
            ctx.fill();
            ctx.closePath();
        }

        update() {
            this.attractToCorners();
            this.attractAndRepelBalls();

            if (this.x + this.radius > canvas.width || this.x - this.radius < 0) {
                this.dx = -this.dx;
            }
            if (this.y + this.radius > canvas.height || this.y - this.radius < 0) {
                this.dy = -this.dy;
            }

            // Ограничение скорости
            const speed = Math.sqrt(this.dx * this.dx + this.dy * this.dy);
            if (speed > MAX_SPEED) {
                this.dx = (this.dx / speed) * MAX_SPEED;
                this.dy = (this.dy / speed) * MAX_SPEED;
            }

            this.x += this.dx;
            this.y += this.dy;
            this.draw();
        }

        attractToCorners() {
            const corners = [
                { x: 0, y: 0 },
                { x: canvas.width, y: 0 },
                { x: 0, y: canvas.height },
                { x: canvas.width, y: canvas.height },
            ];

            corners.forEach(corner => {
                const distX = corner.x - this.x;
                const distY = corner.y - this.y;
                const distance = Math.sqrt(distX * distX + distY * distY);

                if (distance < MAX_ATTRACTION_DISTANCE) {
                    this.dx += (distX / distance) * ATTRACTION_FORCE;
                    this.dy += (distY / distance) * ATTRACTION_FORCE;
                }
            });
        }

        attractAndRepelBalls() {
            balls.forEach(otherBall => {
                if (this === otherBall) return;

                const distX = otherBall.x - this.x;
                const distY = otherBall.y - this.y;
                const distance = Math.sqrt(distX * distX + distY * distY);

                if (distance < MAX_ATTRACTION_DISTANCE) {
                    this.dx += (distX / distance) * MUTUAL_ATTRACTION_FORCE;
                    this.dy += (distY / distance) * MUTUAL_ATTRACTION_FORCE;
                }

                if (distance < this.radius + otherBall.radius) {
                    this.dx -= (distX / distance) * REPULSION_FORCE;
                    this.dy -= (distY / distance) * REPULSION_FORCE;
                }
            });
        }
    }

    function init() {
        balls = [];
        for (let i = 0; i < quantityBalls; i++) {
            let radius = Math.random() * 20 + 10;
            let x = Math.random() * (canvas.width - radius * 2) + radius;
            let y = Math.random() * (canvas.height - radius * 2) + radius;
            let dx = (Math.random() - 0.5) * 2;
            let dy = (Math.random() - 0.5) * 2;
            let color = colorsBalls[Math.floor(Math.random() * colorsBalls.length)];
            balls.push(new Ball(x, y, radius, color, dx, dy));
        }
    }

    function animate() {
        requestAnimationFrame(animate);
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        balls.forEach(ball => ball.update());
    }

    function handleMouseMove(event) {
        const rect = canvas.getBoundingClientRect();
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;

        balls.forEach(ball => {
            const distX = mouseX - ball.x;
            const distY = mouseY - ball.y;
            if (Math.sqrt(distX * distX + distY * distY) < 100) {
                ball.dx = distX / speedBalls;
                ball.dy = distY / speedBalls;
            }
        });
    }

    function handleClick(event) {
        const rect = canvas.getBoundingClientRect();
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;

        balls = balls.filter(ball => {
            const distX = mouseX - ball.x;
            const distY = mouseY - ball.y;
            return Math.sqrt(distX * distX + distY * distY) > ball.radius;
        });
    }

    onMount(() => {
        canvas = document.getElementById('backgroundCanvas');
        ctx = canvas.getContext('2d');
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        init();
        animate();

        window.addEventListener('resize', () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            init();
        });

        window.addEventListener('mousemove', handleMouseMove);
        window.addEventListener('click', handleClick);
    });
</script>

<canvas id="backgroundCanvas"></canvas>

<style>
    canvas {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    }
</style>